use std::fs;
use std::os::unix::fs::MetadataExt;
use std::process::Command;

pub fn load_sleex_user_config() {
    let config_dir = "/etc/skel/.config/";
    let keymap_path = "/etc/vconsole.conf";

    // Get keyboard layout from vconsole.conf
    let vc_keymap = fs::read_to_string(keymap_path)
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|line| line.to_uppercase().starts_with("KEYMAP="))
                .map(|line| line.split('=').nth(1).unwrap_or("").trim_matches('"').to_lowercase())
        })
        .unwrap_or_else(|| "us".to_string());


    if let Ok(entries) = fs::read_dir("/home/") {
        for entry in entries.flatten() {
            let path = entry.path();

            // Vérifie si c’est un dossier utilisateur normal
            if let Ok(meta) = fs::metadata(&path) {
                if meta.uid() >= 1000 && meta.is_dir() {
                    let target_config = path.join(".config");
                    let hypr_custom = target_config.join("hypr/custom/general.conf");
                    let keybinds_dir = target_config.join("hypr/hyprland");

                    println!("Applying config to {:?}", target_config);

                    let sync_cmd = Command::new("rsync")
                        .arg("-a")
                        .arg("--progress")
                        .arg("--exclude=hypr/custom/*")
                        .arg("--exclude=hypr/monitors.conf")
                        .arg("--exclude=hypr/hyprlock.conf")
                        .arg("--exclude=hypr/apps.conf")
                        .arg(format!("{}/", config_dir))
                        .arg(target_config.to_str().unwrap())
                        .output()
                        .expect("Failed to run rsync");

                    if !sync_cmd.status.success() {
                        eprintln!(
                            "Error copying config to {:?}: {}",
                            target_config,
                            String::from_utf8_lossy(&sync_cmd.stderr)
                        );
                        continue;
                    }

                    
                    // Fix ownership: chown -R username:username ~/.config
                    if let Some(user) = path.file_name().and_then(|n| n.to_str()) {
                        let chown_status = Command::new("chown")
                            .arg("-R")
                            .arg(format!("{}:{}", user, user))
                            .arg(target_config.to_str().unwrap())
                            .status();

                        if let Ok(status) = chown_status {
                            if !status.success() {
                                eprintln!("Failed to chown config for user {}", user);
                            }
                        } else {
                            eprintln!("Failed to run chown for user {}", user);
                        }
                    }

                    println!("Setting keyboard layout to '{}' in {:?}", vc_keymap, hypr_custom);

                    // edit or add "input:kb_layout = ..."
                    if hypr_custom.exists() {
                        let content = fs::read_to_string(&hypr_custom).unwrap_or_default();
                        let mut lines: Vec<String> = Vec::new();
                        let mut replaced = false;

                        for line in content.lines() {
                            if line.trim_start().starts_with("input:kb_layout") {
                                lines.push(format!("input:kb_layout = {}", vc_keymap));
                                replaced = true;
                            } else {
                                lines.push(line.to_string());
                            }
                        }

                        if !replaced {
                            lines.push(format!("input:kb_layout = {}", vc_keymap));
                        }

                        fs::write(&hypr_custom, lines.join("\n")).unwrap_or_else(|e| {
                            eprintln!("Failed to write to general.conf: {}", e);
                        });
                    }

                    // Gérer les keybinds
                    println!("Applying keybinds configuration in {:?}", keybinds_dir);

                    let target = keybinds_dir.join("keybinds.conf");
                    let layout_file = match vc_keymap.as_str() {
                        "fr" => keybinds_dir.join("keybinds_fr.conf"),
                        _ => keybinds_dir.join("keybinds_us.conf"),
                    };

                    if layout_file.exists() {
                        fs::rename(&layout_file, &target).unwrap_or_else(|e| {
                            eprintln!("Failed to switch keybinds: {}", e);
                        });
                        println!(
                            "Switched to {} keybinds.",
                            vc_keymap.to_uppercase()
                        );
                    } else {
                        eprintln!("Layout file {:?} not found, skipping keybinds rename.", layout_file);
                    }
                }
            }
        }
    }
}
