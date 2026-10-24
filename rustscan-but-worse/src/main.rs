use std::{env, net::IpAddr, process};

fn main() {
    // Collect the CLI arguments into a Vec<String>
    let args: Vec<String> = env::args().collect();
    // Print the entire vector to see what's inside
    // println!("All arguments: {:?}", args);

    // Confirm In-Line arguments are as many as needed.
    if args.len() < 3 {
        println!("Error: Missing arguments!");
        println!("Usage: cargo run -- <ip_address> <port>");
        process::exit(1)
    }

    // declare variables
    let target: IpAddr = args[1].parse().expect("Invalid IP Address");
    let port: u16 = args[2].parse().expect("Invalid Port numder");
    let service = identify_service(port);

    // scanner(port, target);
    scan_info(target, port, service);
}

// take target address and port and validate value
//fn validate_target() -> {};

//fn validate_port() {};

// matches port value to known services
fn identify_service(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        80 => "HTTP",
        443 => "HTTPS",
        8080 => "Development / Alternative HTTP",
        _ => "Unknown",
    }
}

fn scan_info(target: IpAddr, port: u16, service: &str) {
    println!("\nTarget: {target}");
    println!("Port: {port}");
    println!("Service: {service}\n");
    println!("Initializing scan...\n")
}

/* Placeholder for TCP scanning implementation
fn scanner(port: u16, target: Ipv4Addr) {
    println!("Initializing scan on {target} at port {port}....");
} */
