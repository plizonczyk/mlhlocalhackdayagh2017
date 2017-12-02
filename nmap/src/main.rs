// extern crate libc;
extern crate pnet;

use pnet::datalink;

fn main() {
    // let socket = unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP); };
    let interfaces = datalink::interfaces();
    for interface in interfaces.into_iter() {
        println!("{}", interface.name);
    }
}
