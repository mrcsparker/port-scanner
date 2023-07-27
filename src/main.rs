use clap::Parser;
use pscan::port_scanner::PortScanner;

/// Command-line arguments for the port scanning tool.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host name to scan. Can be specified using the -t or --target options.
    #[arg(short, long, default_value = "127.0.0.1")]
    target: String,
}

fn main() {
    // Parse command-line arguments using Clap.
    let args = Args::parse();

    // Create a new PortScanner instance with the specified configuration.
    // The scanner will scan ports from 1 to 1024 (inclusive) on the target host.
    let scanner = PortScanner::new(1, 1024, args.target);

    // Scan for open ports on the target host using the PortScanner.
    let open_ports = scanner.scan();

    // Print the list of open ports found during the scan.
    println!("Open Ports: {:?}", open_ports);
}
