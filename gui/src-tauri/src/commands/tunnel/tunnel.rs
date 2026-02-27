use std::{
    fs,
    net::Ipv4Addr,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_store::StoreExt;
use vpn_lib::{
    self,
    ssh::{connect_ssh, harden_ssh},
    validate_key_file,
    wireguard::{
        client::list_local_configs,
        server::{build_client_config, setup_wireguard},
    },
};

use crate::{
    commands::{
        tunnel::metadata::{get_all_tunnels, save_metadata_to_store, TunnelMetadata},
        utils::{load_key_securely, save_key_securely},
    },
    TunnelPayload, TunnelState,
};

#[derive(Serialize)]
pub struct ConnectResponse {
    pub config_name: String,
    pub success: bool,
}

#[tauri::command]
pub async fn setup_server(
    app: AppHandle,
    server_ip: String,
    user: String,
    key_file: String,
) -> Result<(), String> {
    println!("{}|{}|{}", server_ip, user, key_file);

    let ip: Ipv4Addr = server_ip
        .parse()
        .map_err(|_| "Invalid IP address format".to_string())?;

    let key_path = PathBuf::from(&key_file);

    validate_key_file(&key_path).map_err(|e| e.to_string())?;

    let session = connect_ssh(ip, user, key_path)
        .await
        .map_err(|e| e.to_string())?;

    let result = setup_wireguard(&session, ip, "eth0".into())
        .await
        .map_err(|e| e.to_string())?;

    let metadata: TunnelMetadata = result.clone().into();

    save_key_securely(result.public_ip, &result.client_private_key)
        .await
        .map_err(|e| e.to_string())?;

    save_metadata_to_store(&app, metadata)?;

    harden_ssh(&session).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn toggle_vpn(connect: bool) -> Result<bool, String> {
    let action = if connect { "up" } else { "down" };
    let interface = "wg0";

    let mut cmd;

    #[cfg(target_os = "windows")]
    {
        cmd = Command::new("wg-quick");
        cmd.arg(action).arg(interface);
    }

    #[cfg(target_os = "linux")]
    {
        cmd = Command::new("pkexec");
        cmd.arg("wg-quick").arg(action).arg(interface);
    }

    #[cfg(target_os = "macos")]
    {
        cmd = Command::new("osascript");
        cmd.arg("-e").arg(format!(
            "do shell script \"wg-quick {} {}\" with administrator privileges",
            action, interface
        ));
    }

    let status = cmd.status().map_err(|e| e.to_string())?;

    Ok(status.success())
}

#[tauri::command]
pub async fn get_configs(app: AppHandle) -> Result<Vec<TunnelMetadata>, String> {
    let tunnels = get_all_tunnels(&app)?;
    Ok(tunnels)
}

#[tauri::command]
pub fn is_tunnel_active(name: String) -> bool {
    #[cfg(target_os = "windows")]
    {
        let service_name = format!("WireGuardTunnel${}", name);
        let output = std::process::Command::new("sc")
            .args(["query", &service_name])
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout.contains("RUNNING")
            }
            Err(_) => false,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let output = std::process::Command::new("wg")
            .args(["show", &name])
            .output();
        output.map(|o| o.status.success()).unwrap_or(false)
    }
}

#[tauri::command]
pub async fn start_tunnel(
    app: AppHandle,
    state: tauri::State<'_, TunnelState>,
    public_ip: Ipv4Addr,
) -> Result<(), String> {
    let store = app
        .store("tunnels.json")
        .map_err(|e| format!("Failed to open store: {}", e))?;

    let public_ip_str = public_ip.to_string();

    let metadata_value = store
        .get(&public_ip_str)
        .ok_or_else(|| format!("No metadata found for {}", public_ip_str))?;

    let client_ip = Ipv4Addr::from_str(metadata_value["client_ip"].as_str().unwrap_or("10.0.0.2")).map_err(|e| e.to_string())?;

    let server_pub_key = metadata_value["server_public_key"].as_str().unwrap_or("");

    let client_private_key =
        load_key_securely(public_ip).map_err(|_| "Failed to load private key")?;

    let wg_config = build_client_config(
        client_private_key.expose_secret(),
        server_pub_key,
        public_ip,
        client_ip,
    );

    let temp_dir = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&temp_dir).ok();
    let temp_config_path = temp_dir.join(format!("{}.conf", public_ip_str));

    fs::write(&temp_config_path, wg_config)
        .map_err(|e| format!("Failed to write temp config: {}", e))?;

    vpn_lib::wireguard::client::start_tunnel(&temp_config_path).map_err(|e| e.to_string())?;

    let _ = fs::remove_file(&temp_config_path);

    let mut active_lock = state.active_tunnel.lock().unwrap();
    *active_lock = Some(public_ip_str.clone());

    app.emit(
        "tunnel-status",
        TunnelPayload {
            name: Some(public_ip_str),
            is_active: true,
        },
    )
    .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn stop_tunnel(
    app: AppHandle,
    state: tauri::State<'_, TunnelState>,
) -> Result<(), String> {
    let tunnel_name = {
        let active_lock = state.active_tunnel.lock().unwrap();
        active_lock.clone()
    };

    let Some(name) = tunnel_name else {
        return Err("No active tunnel found in state".to_string());
    };

    vpn_lib::wireguard::client::stop_tunnel(&name).map_err(|e| e.to_string())?;

    {
        let mut active_lock = state.active_tunnel.lock().unwrap();
        *active_lock = None;
    }

    app.emit(
        "tunnel-status",
        TunnelPayload {
            name: None,
            is_active: false,
        },
    )
    .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn quick_connect(
    app: AppHandle,
    state: State<'_, TunnelState>,
) -> Result<ConnectResponse, String> {
    let configs = get_all_tunnels(&app)?;

    let first_config = configs
        .first()
        .ok_or_else(|| "No VPN configurations found".to_string())?;

    start_tunnel(app, state, first_config.public_ip.clone()).await?;

    Ok(ConnectResponse {
        config_name: first_config.name.clone(),
        success: true,
    })
}
