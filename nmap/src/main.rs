extern crate nmap;

use nmap::{iana, icmp, scans};

use std::net::IpAddr;
use std::process;

fn main() {
    let udp_map = iana::get_udp_map();
    let tcp_map = iana::get_tcp_map();

    let ip_to_scan = match std::env::args().nth(1) {
        Some(n) => n,
        None => {
            println!("Usage: cargo run <ip>");
            process::exit(1);
        }
    };

    let dest_ip = ip_to_scan.parse::<IpAddr>().expect("Invalid IPv4 address");

    println!("Scanning host: {}", ip_to_scan);
    let icmp_result = icmp::icmp_scan(&dest_ip);

    if icmp_result {
        let results = scans::tcp(&ip_to_scan, 0, 65535);
        println!("Checking registered ports (IANA registry)");
        for result in results {
            match tcp_map.get(&(result as u64)) {
                Some(desc) => println!("{}: {}", result, desc),
                _          => continue,
            }
        }
    }
}
