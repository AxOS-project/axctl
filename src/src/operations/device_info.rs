pub fn device_info() {
    use std::process::Command;

    println!("==========[ System Info ]==========");

    let os = Command::new("uname")
        .arg("-o")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "?".into());

    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "?".into());

    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "?".into());

    let memory_raw = Command::new("free")
        .arg("-h")
        .output()
        .expect("Failed to fetch memory info.");
    let memory_output = String::from_utf8_lossy(&memory_raw.stdout);
    let total_memory = memory_output
        .lines()
        .find(|line| line.starts_with("Mem:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("?");

    let cpu_raw = Command::new("lscpu")
        .output()
        .expect("Failed to fetch CPU info.");
    let cpu_output = String::from_utf8_lossy(&cpu_raw.stdout);
    let cpu_model = cpu_output
        .lines()
        .find(|line| line.starts_with("Model name"))
        .and_then(|line| line.split(':').nth(1))
        .map(str::trim)
        .unwrap_or("?");

    let disk_raw = Command::new("df")
        .arg("-h")
        .output()
        .expect("Failed to fetch disk info.");
    let disk_output = String::from_utf8_lossy(&disk_raw.stdout);
    let (disk_used, disk_total): (String, String) = disk_output
        .lines()
        .find(|line| line.contains(" /"))
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts.get(2).unwrap_or(&"?").to_string(),
                parts.get(1).unwrap_or(&"?").to_string(),
            )
        })
        .unwrap_or(("?".into(), "?".into()));

    let hostname = Command::new("hostname")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "?".into());

    let pkg_mgr = Command::new("pacman").arg("-V").output()
    .map(|out| {
        if out.status.success() {
            "pacman (Arch-based)"
        } else {
            Command::new("dpkg").arg("-l").output()
                .map(|out| {
                    if out.status.success() {
                        "dpkg (Debian-based)"
                    } else {
                        "Unknown"
                    }
                })
                .unwrap_or("Unknown")
        }
    })
    .unwrap_or("Unknown");


    println!("Hostname       : {}", hostname);
    println!("OS             : {}", os);
    println!("Kernel         : {}", kernel);
    println!("Uptime         : {}", uptime);
    println!("CPU            : {}", cpu_model);
    println!("RAM (total)    : {}", total_memory);
    println!("Disk (/ used)  : {}/{}", disk_used, disk_total);
    println!("Package Manager: {}", pkg_mgr);
}
