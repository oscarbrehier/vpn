use crate::ssh::{SshClient, SshSession};
use crate::{
    ssh::run_remote_cmd,
    wireguard::{
        peer::Peer,
        state::{VpnState, save_state},
    },
};
use base64::{Engine, engine::general_purpose};
use rand_core::OsRng;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, path::Path};
use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Debug, Clone)]
pub struct SetupResult {
    pub client_private_key: SecretString,
    pub server_public_key: String,
    pub client_ip: Ipv4Addr,
    pub public_ip: Ipv4Addr,
}

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Remote command failed with exit code {status}: {message}")]
    CommandFailed { status: i32, message: String },
    #[error("Server public key not found or empty")]
    KeyNotFound,
    #[error("Internal state error: {0}")]
    State(#[from] crate::wireguard::state::StateError),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TunnelMode {
    Full,
    Split,
}

pub fn generate_keys() -> (SecretString, String) {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    let priv_b64 = general_purpose::STANDARD.encode(secret.to_bytes());
    let pub_b64 = general_purpose::STANDARD.encode(public.to_bytes());

    (SecretString::new(priv_b64.into()), pub_b64)
}

fn build_server_config(
    server_private_key: &str,
    client_public_key: &str,
    interface: &str,
    client_ip: Ipv4Addr,
) -> String {
    format!(
        r#"[Interface]
Address = 10.0.0.1/24
ListenPort = 51820
PrivateKey = {server_private_key}

PostUp = sysctl -w net.ipv4.ip_forward=1; iptables -A FORWARD -i %i -j ACCEPT; iptables -t nat -A POSTROUTING -o {interface} -j MASQUERADE
PostDown = iptables -D FORWARD -i %i -j ACCEPT; iptables -t nat -D POSTROUTING -o {interface} -j MASQUERADE

PostUp = ip6tables -A FORWARD -i %i -j REJECT
PostDown = ip6tables -D FORWARD -i %i -j REJECT

[Peer]
PublicKey = {client_public_key}
AllowedIPs = {client_ip}/32
        "#
    )
}

pub fn build_client_config(
    client_priv: &str,
    server_pub: &str,
    public_ip: Ipv4Addr,
    peer_ip: Ipv4Addr,
    tunnel_mode: &TunnelMode,
) -> String {
    let allowed_ips = match tunnel_mode {
        TunnelMode::Full => "0.0.0.0/0",
        TunnelMode::Split => "0.0.0.0/24",
    };

    format!(
        r#"[Interface]
PrivateKey = {client_priv}
Address = {peer_ip}/32
DNS = 1.1.1.1

[Peer]
PublicKey = {server_pub}
Endpoint = {public_ip}:51820
AllowedIPs = {allowed_ips}
"#
    )
}

pub async fn upload_file(ssh_client: &SshClient, path: &Path, content: &str) -> anyhow::Result<()> {
    let b64_content = general_purpose::STANDARD.encode(content);
    let cmd = format!(
        "echo '{}' | base64 -d | {} tee {} > /dev/null",
        b64_content,
        ssh_client.sudo_prefix,
        path.display()
    );

    let (output, status) = ssh_client.exec_raw(&cmd).await?;

    if status != 0 {
        anyhow::bail!("Failed to upload file to {}: {}", path.display(), output);
    }

    ssh_client
        .exec(&format!("chmod 600 {}", path.display()))
        .await?;

    Ok(())
}

pub async fn setup_wireguard(
    ssh_client: &SshClient,
    public_ip: Ipv4Addr,
    interface: &str,
) -> anyhow::Result<SetupResult> {
    let (_, status) = ssh_client.exec_raw("which wg").await?;

    if status != 0 {
        let apt_cmd = "DEBIAN_FRONTEND=noninteractive apt-get update -y && \
                   DEBIAN_FRONTEND=noninteractive apt-get install -y -q wireguard iptables";

        let (output, install_status) = ssh_client.exec(apt_cmd).await?;

        if install_status != 0 {
            println!("Wireguard installation failed: {}", output);
            anyhow::bail!("Wireguard installation failed");
        }
    }
    let (server_priv, server_pub) = generate_keys();
    let (new_peer, peer_priv_key) = Peer::new("initial-client".into(), Ipv4Addr::new(10, 0, 0, 2));

    let mut state = VpnState::new(server_pub.clone(), public_ip);
    state.peers.push(new_peer.clone());

    let server_config = build_server_config(
        &server_priv.expose_secret(),
        &new_peer.public_key,
        interface,
        new_peer.ip,
    );

    ssh_client.exec("mkdir -p /etc/wireguard").await?;
    ssh_client.exec("chmod 700 /etc/wireguard").await?;

    let config_path = Path::new("/etc/wireguard/wg0.conf");
    upload_file(ssh_client, config_path, &server_config).await?;

    ssh_client.exec("{} wg-quick down wg0 || true").await?;
    ssh_client.exec("{} wg-quick up wg0").await?;

    save_state(ssh_client, &state).await?;

    Ok(SetupResult {
        client_private_key: peer_priv_key,
        server_public_key: server_pub,
        client_ip: new_peer.ip,
        public_ip,
    })
}

pub async fn update_wireguard_config(
    ssh_client: &SshClient,
    state: &VpnState,
) -> anyhow::Result<()> {
    let (current_peers_raw, _) = ssh_client.exec("wg show wg0 peers").await?;
    let active_keys: Vec<&str> = current_peers_raw.lines().collect();

    for key in active_keys {
        if !key.is_empty() && !state.peers.iter().any(|p| p.public_key == key) {
            ssh_client
                .exec(&format!("wg set wg0 peer {} remove", key))
                .await?;
        }
    }

    for peer in &state.peers {
        ssh_client
            .exec(&format!(
                "wg set wg0 peer {} allowed-ips {}/32",
                peer.public_key, peer.ip
            ))
            .await?;
    }

    ssh_client.exec("wg-quick save wg0").await?;

    anyhow::Ok(())
}

pub async fn get_server_public_key(ssh_client: &SshClient) -> anyhow::Result<String> {
    let (pub_key, status) = ssh_client.exec("wg show wg0 public-key").await?;

    if status != 0 {
        return Err(ServerError::CommandFailed {
            status,
            message: pub_key,
        }
        .into());
    }

    let trimmed_key = pub_key.trim();

    if trimmed_key.is_empty() {
        return Err(ServerError::KeyNotFound.into());
    }

    anyhow::Ok(pub_key.to_string())
}
