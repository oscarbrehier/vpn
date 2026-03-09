#[cfg(target_os = "windows")]
use vpn_lib::app_filter::scanner::get_windows_icon;
use vpn_lib::app_filter::scanner::{AppGroup, get_running_apps};

use crate::AppCache;

#[tauri::command]
pub async fn fetch_apps(
	cache: tauri::State<'_, AppCache>
) -> Result<Vec<AppGroup>, String> {

	get_running_apps(|path| {

		let path_str = path.to_string_lossy().to_string();

		if let Some(icon) = cache.icons.get(&path_str) {
			return Some(icon.value().clone());
		}

		#[cfg(target_os = "windows")]
		if let Some(new_icon) = get_windows_icon(path) {
			cache.icons.insert(path_str, new_icon.clone());
			return Some(new_icon);
		}

		None

	})

}