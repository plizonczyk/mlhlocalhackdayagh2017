extern crate pnet;
extern crate nmap;

use pnet::datalink::{self, interfaces, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

use nmap::iana;
use nmap::scans;
use std::process;


fn main() {
    let udp_map = iana::get_udp_map();
    let tcp_map = iana::get_tcp_map();

    let given_iface = match std::env::args().nth(1) {
        Some(n) => n,
        None => {
            println!("Usage: cargo run <interface> <ip>");
            process::exit(1);
        }
    };

    let ip_to_scan = match std::env::args().nth(2) {
        Some(n) => n,
        None => {
            println!("Usage: cargo run <interface> <ip>");
            process::exit(1);
        }
    };

    let cmp_fn = |interface: &NetworkInterface| interface.name == given_iface;
    let iface = match interfaces().into_iter().filter(cmp_fn).next() {
        Some(n) => n,
        None => {
            println!("Available interfaces:");
            for interface in interfaces().into_iter() {
                println!("{}", interface.name);
            }
            process::exit(1);
        }
    };

    println!("Scanning host: {}", ip_to_scan);
    let results = scans::tcp(&ip_to_scan, 0, 65535);
    println!("Checking registered ports (IANA registry)");
    for result in results {
        match tcp_map.get(&(result as u64)) {
            Some(desc) => println!("{}: {}", result, desc),
            _          => continue,
        }
    }
}
