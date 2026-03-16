use tauri::AppHandle;

use crate::commands::tunnel::metadata::{TunnelMetadata, get_all_tunnels, remove_metadata_from_store};

#[tauri::command]
pub async fn get_configs(app: AppHandle) -> Result<Vec<TunnelMetadata>, String> {
    let tunnels = get_all_tunnels(&app)?;
    Ok(tunnels)
}

#[tauri::command]
pub fn remove_config(app: AppHandle, key: String) -> Result<(), String> {
	remove_metadata_from_store(&app, key)
}