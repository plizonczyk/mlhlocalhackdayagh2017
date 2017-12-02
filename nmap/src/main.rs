extern crate nmap;

use nmap::{iana, icmp};

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

    let index = 4553;
    let udp_result = match udp_map.get(&index) {
        Some(res) => res,
        None      => "unknown",
    };

    let tcp_result = match tcp_map.get(&index) {
        Some(res) => res,
        None      => "unknown",
    };

    println!("udp: {}", udp_result);
    println!("tcp: {}", tcp_result);

    println!("Scanning {}", &ip_to_scan);
    icmp::icmp_scan(&dest_ip);
}
