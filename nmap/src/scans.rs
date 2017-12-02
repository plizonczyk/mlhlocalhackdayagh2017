use std::net;
use std::cmp;

pub fn udp(host: &str, first: u16, last: u16) -> Vec<u32> {
    let mut result = Vec::new();

    let socket = net::UdpSocket::bind("0.0.0.0:0").unwrap();

    for port_num in first..last {
        let port_str = port_num.to_string();
        let addr = host.to_string() + ":" + &port_str;
        println!("Testing: {}", addr);

        let bytes_amount = match socket.send_to(&[63], addr) {
            Ok(res) => res,
            Err(_)    => continue,
        };

        println!("Bytes: {}", bytes_amount);

        match bytes_amount.cmp(&0) {
            cmp::Ordering::Greater => {
                result.push(port_num as u32);
            },
            _ => continue,
        }
        // println!("{}", bytes_amount)
    }
    result
}

pub fn tcp(host: &str, first: u16, last: u16) -> Vec<u32> {
    let mut result = Vec::new();

    for port_num in first..last {
        let port_str = port_num.to_string();
        let addr = host.to_string() + ":" + &port_str;
        
        match net::TcpStream::connect(&addr) {
            Ok(_) => result.push(port_num as u32),
            Err(_)     => continue,
            // Err(_) => {
            //     println!("No match {}", addr)
            // }
        }
    }
    result
}
