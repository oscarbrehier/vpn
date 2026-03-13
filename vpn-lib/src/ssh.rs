use anyhow::Context;
use russh::client::{AuthResult, Config, Handle, Handler};
use russh::keys::ssh_key;
use std::{net::Ipv4Addr, path::PathBuf, sync::Arc};

use crate::SshError;

pub struct ClientHandler;

impl Handler for ClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> std::result::Result<bool, Self::Error> {
        std::result::Result::Ok(true)
    }
}

pub type SshSession = Handle<ClientHandler>;

pub struct SshClient {
    pub session: SshSession,
    pub sudo_prefix: String,
}

impl SshClient {
    pub fn new(session: SshSession, user: &str) -> Self {
        let sudo_prefix = if user == "root" { "" } else { "sudo " }.to_string();
        Self {
            session,
            sudo_prefix,
        }
    }

    pub async fn exec(&self, cmd: &str) -> anyhow::Result<(String, i32)> {
        let full_cmd = format!("{}{}", self.sudo_prefix, cmd);
        run_remote_cmd(&self.session, &full_cmd).await
    }

    pub async fn exec_raw(&self, cmd: &str) -> anyhow::Result<(String, i32)> {
        run_remote_cmd(&self.session, cmd).await
    }
}

pub async fn connect_ssh(
    addr: Ipv4Addr,
    port: u16,
    user: String,
    key_path: PathBuf,
) -> std::result::Result<SshClient, SshError> {
    let config = Config::default();
    let config = Arc::new(config);
    let sh = ClientHandler;

    let key_pair = russh::keys::load_secret_key(key_path, None)
        .map_err(|e| SshError::HandshakeFailed(format!("Failed to load key: {}", e)))?;

    let mut session = russh::client::connect::<ClientHandler, _>(config, (addr, port), sh)
        .await
        .map_err(|e| SshError::HandshakeFailed(format!("Connection failed: {}", e)))?;

    let key_with_alg = russh::keys::PrivateKeyWithHashAlg::new(Arc::new(key_pair), None);

    let auth_res = session
        .authenticate_publickey(user.clone(), key_with_alg)
        .await
        .map_err(|e| SshError::HandshakeFailed(format!("Auth request failed: {}", e)))?;

    match auth_res {
        AuthResult::Success => {
            let client = SshClient::new(session, &user);
            std::result::Result::Ok(client)
        }
        _ => std::result::Result::Err(SshError::AuthFailed("Access denied".into())),
    }
}

pub async fn run_remote_cmd(session: &SshSession, cmd: &str) -> anyhow::Result<(String, i32)> {
    let mut channel = session
        .channel_open_session()
        .await
        .context("Failed to open SSH channel")?;

    channel.exec(true, cmd).await?;

    let mut output = String::new();
    let mut exit_code = 0;

    while let Some(msg) = channel.wait().await {
        match msg {
            russh::ChannelMsg::Data { ref data } => {
                output.push_str(&String::from_utf8_lossy(data));
            }
            russh::ChannelMsg::ExtendedData { ref data, .. } => {
                output.push_str(&String::from_utf8_lossy(data));
            }
            russh::ChannelMsg::ExitStatus { exit_status } => {
                exit_code = exit_status as i32;
            }
            russh::ChannelMsg::Close => break,
            _ => {}
        }
    }

    Ok((output, exit_code))
}

pub async fn harden_ssh(ssh_client: &SshClient) -> anyhow::Result<()> {
    ssh_client.exec("sed -i 's/^#\\?PasswordAuthentication .*/PasswordAuthentication no/' /etc/ssh/sshd_config").await?;
    ssh_client.exec("sed -i 's/^#\\?ChallengeResponseAuthentication .*/ChallengeResponseAuthentication no/' /etc/ssh/sshd_config").await?;

    let restart_cmd = format!(
        "(sleep 1 && {} systemctl restart ssh) > /dev/null 2>&1 &",
        ssh_client.sudo_prefix
    );
    ssh_client.exec_raw(&restart_cmd).await?;

    anyhow::Ok(())
}
