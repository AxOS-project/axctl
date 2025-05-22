use std::{fs, path::Path};
use std::process::Command;

pub fn load_style_config() {
    let config_dir = "/etc/skel/.config/";

    if let Ok(entries) = fs::read_dir("/home/") {
        for entry in entries.flatten() {
            let path = entry.path();
            let target_dir = path.join(".config");

            if target_dir.exists() || path.is_dir() {
                println!("Applying config to {:?}", target_dir);

                let output = Command::new("rsync")
                    .arg("-a")
                    .arg("--progress")
                    .arg("--exclude=hypr/custom/*")
                    .arg(config_dir)
                    .arg(target_dir.to_str().unwrap())
                    .output()
                    .expect("Failed to run rsync");

                if !output.status.success() {
                    eprintln!(
                        "Error copying config to {:?}: {}",
                        target_dir,
                        String::from_utf8_lossy(&output.stderr)
                    );
                } else {
                    println!("Config applied to {:?}", target_dir);
                }
            }
        }
    }
}
