use std::fs;
use std::os::unix::fs::MetadataExt;
use std::process::Command;

pub fn load_sleex_user_config() {
    let config_dir = "/etc/skel/.config/";

    if let Ok(entries) = fs::read_dir("/home/") {
        for entry in entries.flatten() {
            let path = entry.path();

            // Vérifie si c’est un dossier utilisateur normal
            if let Ok(meta) = fs::metadata(&path) {
                if meta.uid() >= 1000 && meta.is_dir() {
                    let target_config = path.join(".config");
                    println!("Applying config to {:?}", target_config);

                    let output = Command::new("rsync")
                        .arg("-a")
                        .arg("--progress")
                        .arg("--exclude=hypr/custom/*")
                        .arg(format!("{}/", config_dir)) 
                        .arg(target_config.to_str().unwrap())
                        .output()
                        .expect("Failed to run rsync");

                    if !output.status.success() {
                        eprintln!(
                            "Error copying config to {:?}: {}",
                            target_config,
                            String::from_utf8_lossy(&output.stderr)
                        );
                    } else {
                        println!("Config applied to {:?}", target_config);
                    }
                }
            }
        }
    }
}
