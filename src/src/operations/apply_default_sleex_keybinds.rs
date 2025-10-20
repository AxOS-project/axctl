use std::fs;
use std::path::PathBuf;

pub fn apply_default_sleex_keybinds() {
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


    let keybinds_dir = PathBuf::from("/etc/sleex/hyprland");

    // Manage keybinds
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
