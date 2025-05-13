use crate::internal::utils;

pub async fn splash_screen_toggle() {

    const ACTIVE_CONFIG: &str = "/etc/plymouth/plymouthd.conf";
    
    let status;
    let file_content = std::fs::read_to_string(ACTIVE_CONFIG).unwrap();
    
    if file_content.contains("Theme=axos") {
        utils::sed_in_file(ACTIVE_CONFIG, "Theme=axos", "Theme=details").unwrap();
        status = "details";
    } else if file_content.contains("Theme=details") {
        utils::sed_in_file(ACTIVE_CONFIG, "Theme=details", "Theme=axos").unwrap();
        status = "axos";
    } else {
        eprintln!("No valid theme found in {}. Valid themes are: axos, details", ACTIVE_CONFIG);
        return;
    }
    
    let output = std::process::Command::new("update-initramfs")
        .arg("-u")
        .arg("-k")
        .arg("all")
        .output()
        .expect("Failed to update Plymouth configuration");
    
    if !output.status.success() {
        eprintln!("Error updating Plymouth configuration: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Successfully changed splash screen to {}.", status);
    }
}