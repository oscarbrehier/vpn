use crate::commands::tunnel::RedirectionState;

#[tauri::command]
pub async fn update_selected_apps(state: tauri::State<'_, RedirectionState>, pids: Vec<u32>) -> Result<(), String> {

	let guard = state.filter_rx.lock().await;

	if let Some(tx) = &*guard {
		tx.send(pids).map_err(|e| e.to_string())?;
		Ok(())
	} else {
		Err("No active redirection loop".into())
	}

}