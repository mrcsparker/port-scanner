use rayon::prelude::*;
use std::net::{TcpStream, UdpSocket};

struct PortScanner {
    start_port: u16,
    end_port: u16,
    target_ip: String,
}

impl PortScanner {
    fn new(start_port: u16, end_port: u16, target_ip: String) -> PortScanner {
        PortScanner {
            start_port,
            end_port,
            target_ip,
        }
    }

    fn scan(&self) -> Vec<u16> {
        (self.start_port..self.end_port + 1)
            .into_par_iter()
            .filter_map(|port| {
                let tcp_check = TcpStream::connect(format!("{}:{}", self.target_ip, port));
                let udp_check = UdpSocket::bind(format!("{}:{}", self.target_ip, port));

                if tcp_check.is_ok() || udp_check.is_ok() {
                    Some(port)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let scanner = PortScanner::new(1, 1024, String::from("127.0.0.1"));
    let open_ports = scanner.scan();
    println!("Open Ports: {open_ports:?}");
}
