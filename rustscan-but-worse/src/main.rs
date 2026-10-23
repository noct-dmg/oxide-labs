use std::env;
// use std::net::Ipv4Addr;

fn main() {
    // Collect the CLI arguments into a Vec<String>
    let args: Vec<String> = env::args().collect();
    /* Print the entire vector to see what's inside
    println!("All arguments: {:?}", args);
    */
    if args.len() < 3 {
        println!("Error: Missing arguments!");
        println!("Usage: cargo run -- <ip_address> <port>");
        std::process::exit(1)
    }

    let target = "127.0.0.1";
    let port: u16 = 80;

    let service = identify_service(port);

    println!("The service most commonly associated with {port} is {service}");
    scanner(port, target);
}

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
// Placeholder for TCP scanning implementation
fn scanner(port: u16, target: &str) {
    println!("Initializing scan on {target} at port {port}....");
}
