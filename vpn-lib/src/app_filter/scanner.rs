use std::path::PathBuf;

use serde::{Serialize};
use sysinfo::{System};

#[derive(Debug, Serialize)]
pub struct AppProcess {
    pub pid: u32,
    pub name: String,
    pub path: Option<PathBuf>,
    pub icon_base64: Option<String>,
}

pub fn get_running_apps() -> Result<Vec<AppProcess>, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<AppProcess> = Vec::new();

    for (pid, process) in sys.processes() {

		let pid_val = pid.as_u32();
		let name = process.name().to_string_lossy().into_owned();
		let path = process.exe().map(|p| p.to_path_buf());

        processes.push(AppProcess {
            pid: pid_val,
            name: name,
            path: path,
            icon_base64: None,
        });
    }

    Ok(processes)
}
