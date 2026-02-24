use std::{fs, path::Path};

pub fn list_local_configs(conf_dir: &Path) -> anyhow::Result<Vec<String>> {

	let mut configs = Vec::new();

	if conf_dir.exists() && conf_dir.is_dir() {

		for entry in fs::read_dir(conf_dir)? {

			let entry = entry?;
			let path = entry.path();

			if path.extension().and_then(|s| s.to_str()) == Some("conf") {
				if let Some(file_stem) = path.file_name().and_then(|s| s.to_str()) {
					configs.push(file_stem.to_string());
				}
			}

		}

	}

	anyhow::Ok(configs)

}