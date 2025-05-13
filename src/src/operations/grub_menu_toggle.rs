use crate::internal::utils;

pub async fn grub_menu_toggle() {
    const GRUB_CONFIG: &str = "/etc/default/grub";
    
    let file_content = std::fs::read_to_string(GRUB_CONFIG).unwrap();
    
    if file_content.contains("GRUB_TIMEOUT_STYLE=\"menu\"") {
        utils::sed_in_file(GRUB_CONFIG, "GRUB_TIMEOUT_STYLE=\"menu\"", "GRUB_TIMEOUT_STYLE=\"hidden\"").unwrap();
    } else {
        utils::sed_in_file(GRUB_CONFIG, "GRUB_TIMEOUT_STYLE=\"hidden\"", "GRUB_TIMEOUT_STYLE=\"menu\"").unwrap();
    }
    
    // Update the GRUB configuration
    let output = std::process::Command::new("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .output()
        .expect("Failed to update GRUB configuration");
    
    if !output.status.success() {
        eprintln!("Error updating GRUB configuration: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Successfully toggled menu visibility.");
    }
}