use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let sidecar_src = PathBuf::from("../../target")
        .join(&profile)
        .join(if cfg!(windows) {
            "sidecar.exe"
        } else {
            "sidecar"
        });

    if !sidecar_src.exists() {
        panic!("Sidecar binary not found");
    }

    let ext = if target.contains("windows") {
        ".exe"
    } else {
        ""
    };
    let sidecar_name = format!("sidecar-{}{}", target, ext);

    std::fs::create_dir_all("binaries").unwrap();
    let bundle_path = PathBuf::from("binaries").join(&sidecar_name);
    std::fs::copy(&sidecar_src, &bundle_path).expect("Failed to copy to binaries");

    let dev_path = PathBuf::from("../../target")
        .join(&profile)
        .join(&sidecar_name);
    std::fs::copy(&sidecar_src, &dev_path).ok();

    println!("cargo:rerun-if-changed=../../sidecar");

    tauri_build::build();
}
