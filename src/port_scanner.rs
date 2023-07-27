use rayon::prelude::*;
use std::net::{TcpStream, UdpSocket};

/// Represents a simple port scanner that can scan for open ports on a given target IP.
pub struct PortScanner {
    /// The starting port number for scanning.
    pub start_port: u16,

    /// The ending port number for scanning (inclusive).
    pub end_port: u16,

    /// The IP address of the target host to scan for open ports.
    pub target_ip: String,
}

impl PortScanner {
    /// Creates a new `PortScanner` instance with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `start_port`: The starting port number for scanning.
    /// * `end_port`: The ending port number for scanning (inclusive).
    /// * `target_ip`: The IP address of the target host to scan for open ports.
    ///
    /// # Returns
    ///
    /// A new `PortScanner` instance.
    pub fn new(start_port: u16, end_port: u16, target_ip: String) -> PortScanner {
        PortScanner {
            start_port,
            end_port,
            target_ip,
        }
    }

    /// Scans for open ports in the specified range on the target IP using TCP and UDP connections.
    ///
    /// # Returns
    ///
    /// A vector containing the open port numbers found during the scan.
    pub fn scan(&self) -> Vec<u16> {
        // Use Rayon to parallelize the port scanning process for faster execution.
        (self.start_port..=self.end_port) // Range from start_port to end_port (inclusive).
            .into_par_iter() // Convert the range into a parallel iterator.
            .filter_map(|port| {
                // Try to establish a TCP connection to the target IP and port.
                let tcp_check = TcpStream::connect(format!("{}:{}", self.target_ip, port));

                // Try to bind a UDP socket to the target IP and port.
                let udp_check = UdpSocket::bind(format!("{}:{}", self.target_ip, port));

                // If either the TCP connection or UDP binding is successful, the port is open.
                // Return the port number as Some(port), otherwise None.
                if tcp_check.is_ok() || udp_check.is_ok() {
                    Some(port)
                } else {
                    None
                }
            })
            .collect() // Collect the results of the parallel iterator into a vector.
    }
}
