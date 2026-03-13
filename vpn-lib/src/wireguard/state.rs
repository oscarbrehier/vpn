use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

use crate::{
    ssh::{SshClient, SshSession, run_remote_cmd},
    wireguard::{peer::Peer, server},
};

#[derive(Deserialize, Debug, Serialize)]
pub struct VpnState {
    pub server_public_key: String,
    pub server_ip: Ipv4Addr,
    pub peers: Vec<Peer>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("The network is full (maximum 253 peers reached)")]
    NetworkFull,
}

impl VpnState {
    fn default() -> Self {
        Self {
            server_public_key: String::new(),
            server_ip: Ipv4Addr::new(0, 0, 0, 0),
            peers: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    pub fn new(server_public_key: String, server_ip: Ipv4Addr) -> Self {
        Self {
            server_public_key,
            server_ip,
            peers: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    pub fn get_next_available_ip(&self) -> Result<Ipv4Addr, StateError> {
        let base_ip = [10, 0, 0, 0];

        let max_octet = self
            .peers
            .iter()
            .map(|p| p.ip.octets()[3])
            .max()
            .unwrap_or(1);

        if max_octet >= 254 {
            return Err(StateError::NetworkFull);
        }

        Ok(Ipv4Addr::new(
            base_ip[0],
            base_ip[1],
            base_ip[2],
            max_octet + 1,
        ))
    }
}

pub async fn get_or_create_state(
    ssh_client: &SshClient,
    server_ip: Ipv4Addr,
) -> anyhow::Result<VpnState> {
    let cmd = "cat /etc/wireguard/peers.json";

    let (output, status) = ssh_client.exec_raw(&cmd).await?;

    if status != 0 || output.trim().is_empty() {
        let server_pub = server::get_server_public_key(ssh_client).await?;
        Ok(VpnState::new(server_pub, server_ip))
    } else {
        Ok(serde_json::from_str(&output)?)
    }
}

pub async fn save_state(ssh_client: &SshClient, state: &VpnState) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(state)?;
    let escaped_json = json.replace("'", "'\\''");

    let cmd = format!(
        "echo '{}' | {} tee /etc/wireguard/peers.json > /dev/null",
        escaped_json, ssh_client.sudo_prefix
    );

    let (output, status) = ssh_client.exec_raw(&cmd).await?;

    if status != 0 {
        return Err(anyhow::anyhow!(
            "Failed to save state to server. Exit code: {}. Error: {}",
            status,
            output
        ));
    }

    anyhow::Ok(())
}
