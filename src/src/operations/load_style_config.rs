use std::env;

pub async fn load_style_config() {

    let current_user: &str = &env::var("SUDO_USER").unwrap_or_else(|_| "Unknown".to_string());

    const CONFIG_DIR: &str = "/etc/skel/.config/*";
    const TARGET_DIR: &str = &format!("/home/{}/.config/", current_user);

    // using rsync instead of cp because it is more efficient
    let output = std::process::Command::new("rsync")
        .arg("-a")
        .arg("--progress")
        .arg(CONFIG_DIR)
        .arg(TARGET_DIR)
        .output()
        .expect("Failed to copy configuration files");

    if !output.status.success() {
        eprintln!("Error copying configuration files: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Successfully copied configuration files to {}.", TARGET_DIR);
    }
}