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

pub async fn connect_ssh(
    addr: Ipv4Addr,
    user: String,
    key_path: PathBuf,
) -> std::result::Result<SshSession, SshError> {
    let config = Config::default();
    let config = Arc::new(config);
    let sh = ClientHandler;

    let key_pair = russh::keys::load_secret_key(key_path, None)
        .map_err(|e| SshError::HandshakeFailed(format!("Failed to load key: {}", e)))?;

    let mut session = russh::client::connect::<ClientHandler, _>(config, (addr, 22), sh)
        .await
        .map_err(|e| SshError::HandshakeFailed(format!("Connection failed: {}", e)))?;

    let key_with_alg = russh::keys::PrivateKeyWithHashAlg::new(Arc::new(key_pair), None);

    let auth_res = session
        .authenticate_publickey(user, key_with_alg)
        .await
        .map_err(|e| SshError::HandshakeFailed(format!("Auth request failed: {}", e)))?;

    match auth_res {
        AuthResult::Success => std::result::Result::Ok(session),
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
            russh::ChannelMsg::ExitStatus { exit_status } => {
                exit_code = exit_status as i32;
            }
            russh::ChannelMsg::Close => break,
            _ => {}
        }
    }

    Ok((output, exit_code))
}

pub async fn harden_ssh(session: &SshSession) -> anyhow::Result<()> {
    println!("disabling password authentication");

    let cmd = r#"
        sudo sed -i 's/^#\?PasswordAuthentication .*/PasswordAuthentication no/' /etc/ssh/sshd_config && \
        sudo sed -i 's/^#\?ChallengeResponseAuthentication .*/ChallengeResponseAuthentication no/' /etc/ssh/sshd_config && \
        (sleep 1 && sudo systemctl restart ssh) > /dev/null 2>&1 &
        echo "DONE"
    "#;

    let (output, _) = run_remote_cmd(session, cmd).await?;

    if output.contains("DONE") {
        println!("SSH now locked to key-only access (restarting in 1s)");
    } else {
        println!("SSH command sent, but verify the restart manually");
    };

    anyhow::Ok(())
}
