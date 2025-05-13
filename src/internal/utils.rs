
pub fn is_run_with_root() -> bool {
    let uid = unsafe { libc::geteuid() };
    uid == 0
}

pub fn sed_in_file(file_path: &str, search: &str, replace: &str, ) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let new_content = file_content.replace(search, replace);
    std::fs::write(file_path, new_content)?;
    Ok(())
}