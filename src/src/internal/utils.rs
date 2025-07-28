
pub fn is_run_with_root() -> bool {
    let uid = unsafe { libc::geteuid() };
    uid == 0
}

pub fn sed_in_file(file_path: &str, search: &str, replace: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(file_path)?;
    let lines: Vec<String> = file
        .lines()
        .map(|line| line.replace(search, replace))
        .collect();
    let new_content = lines.join("\n");
    std::fs::write(file_path, new_content)?;
    Ok(())
}
