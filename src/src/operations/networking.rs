use crate::args::SimpleNetworking;
use crate::args::NetworkOperation;
use std::process::Command;

pub fn networking(args: &SimpleNetworking) {
    match &args.action {
        NetworkOperation::Status => {
            network_status();
        }
        NetworkOperation::Restart => {
            network_restart();
        }
        NetworkOperation::Test => {
            network_test();
        }
    }
}

fn network_status() {
    println!("==================== Network Status ====================");

    // interface details
    println!("\n--- Network Interfaces ---");
    let ip_output = Command::new("ip")
        .arg("a")
        .output()
        .expect("Failed to run 'ip a'");

    if !ip_output.status.success() {
        eprintln!("Error running 'ip a': {}", String::from_utf8_lossy(&ip_output.stderr));
        return;
    }
    println!("{}", String::from_utf8_lossy(&ip_output.stdout));

    // interface status
    println!("\n--- Interface Status ---");
    let link_output = Command::new("ip")
        .arg("link")
        .output()
        .expect("Failed to run 'ip link'");

    if !link_output.status.success() {
        eprintln!("Error running 'ip link': {}", String::from_utf8_lossy(&link_output.stderr));
        return;
    }
    println!("{}", String::from_utf8_lossy(&link_output.stdout));

    // our routing table
    println!("\n--- Routing Table ---");
    let route_output = Command::new("ip")
        .arg("route")
        .output()
        .expect("Failed to run 'ip route'");

    if !route_output.status.success() {
        eprintln!("Error running 'ip route': {}", String::from_utf8_lossy(&route_output.stderr));
        return;
    }
    println!("{}", String::from_utf8_lossy(&route_output.stdout));

    // dns
    println!("\n--- DNS Configuration ---");
    let dns_output = std::fs::read_to_string("/etc/resolv.conf")
        .unwrap_or_else(|_| "Unable to read /etc/resolv.conf".to_string());
    println!("{}", dns_output);

    println!("\n--- Active Connections ---");
    let netstat_output = Command::new("ss")
        .arg("-tuln")
        .output()
        .expect("Failed to run 'ss'");

    if !netstat_output.status.success() {
        eprintln!("Error running 'ss': {}", String::from_utf8_lossy(&netstat_output.stderr));
        return;
    }
    println!("{}", String::from_utf8_lossy(&netstat_output.stdout));

    // ping
    println!("\n--- Internet Connectivity Test ---");
    let ping_output = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg("8.8.8.8")
        .output()
        .expect("Failed to run 'ping'");

    if !ping_output.status.success() {
        eprintln!("Error pinging: {}", String::from_utf8_lossy(&ping_output.stderr));
        return;
    }
    println!("{}", String::from_utf8_lossy(&ping_output.stdout));

    println!("\n--- Packet Statistics ---");
    let stats_output = Command::new("netstat")
        .arg("-i")
        .output()
        .expect("Failed to run 'netstat -i'");

    if !stats_output.status.success() {
        eprintln!("Error running 'netstat -i': {}", String::from_utf8_lossy(&stats_output.stderr));
        return;
    }
    println!("{}", String::from_utf8_lossy(&stats_output.stdout));

    println!("==================== End of Network Status ====================");

    // tbh idk what 60% of the stuff here means, it just outputs some random junk but i think it makes sense.
    // ¯\_(ツ)_/¯
}

fn network_restart() {
    println!("Restarting networking services...");

    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("restart")
        .arg("NetworkManager") // google said this is the one we should restart so ig it works? i didnt test it.
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Networking services restarted successfully.");
        }
        Ok(output) => {
            eprintln!("Error restarting networking services: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}

fn network_test() {
    // idk why i added this but the same info can be found in status
    println!("==================== Network Diagnostics ====================");

    println!("\n--- Internet Connectivity Test (ping) ---");
    let ping_output = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg("8.8.8.8")
        .output();
    
    match ping_output {
        Ok(output) if output.status.success() => {
            println!("Ping successful! Internet is reachable.");
        }
        Ok(output) => {
            eprintln!("Ping failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Error running ping: {}", e);
        }
    }

    println!("\n--- Open Network Connections ---");
    let ss_output = Command::new("ss")
        .arg("-tuln")
        .output();

    match ss_output {
        Ok(output) if output.status.success() => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Ok(output) => {
            eprintln!("Error checking open connections: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Error running ss: {}", e);
        }
    }

    println!("==================== End of Network Diagnostics ====================");
}
