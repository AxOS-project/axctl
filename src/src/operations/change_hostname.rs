pub fn change_hostname(hostname: &str) {
    let output = std::process::Command::new("sudo")
        .arg("hostnamectl")
        .arg("set-hostname")
        .arg(hostname)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Hostname changed to: {}", hostname);
        }
        Ok(output) => {
            eprintln!(
                "Failed to change hostname. Error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        Err(e) => {
            eprintln!("Error running hostnamectl: {}", e);
            return;
        }
    }

    let hosts_path = "/etc/hosts";
    let hosts_content = match std::fs::read_to_string(hosts_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", hosts_path, e);
            return;
        }
    };

    let old_hostname = match get_current_hostname() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Failed to get current hostname: {}", e);
            return;
        }
    };

    let updated_hosts = hosts_content.replace(
        &format!("127.0.1.1\t{}", old_hostname),
        &format!("127.0.1.1\t{}", hostname),
    );

    if let Err(e) = std::fs::write(hosts_path, updated_hosts) {
        eprintln!("Failed to write to {}: {}", hosts_path, e);
        return;
    }

    println!("Updated /etc/hosts with new hostname.");
}

fn get_current_hostname() -> Result<String, String> {
    let output = std::process::Command::new("hostname")
        .output()
        .map_err(|e| format!("Failed to run hostname command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
