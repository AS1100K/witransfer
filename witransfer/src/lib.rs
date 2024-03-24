use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

pub fn discover(receiving_addr: &String, port: u16) {
    let broadcast_addr = "225.225.225.225";

    // Create a UDP socket for sending and receiving discovery messages
    let socket = UdpSocket::bind(format!("{receiving_addr}:{port}")).expect("Failed to bind socket");
    socket.set_broadcast(true).expect("Failed to set broadcast option");

    // Send a discovery message
    let discovery_message = "WiTransfer Discover";
    let broadcast_dest = SocketAddr::new(IpAddr::V4(broadcast_addr.parse().unwrap()), port);
    socket.send_to(discovery_message.as_bytes(), broadcast_dest).expect("Failed to send discovery message");

    // Set a timeout for receiving responses
    socket.set_read_timeout(Some(Duration::from_secs(5))).expect("Failed to set read timeout");

    // Listen for response from other users
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let response = std::str::from_utf8(&buf[..size]).expect("Invalid UTF-8 data");
                println!("Discovered user: {} ({})", response, addr)
            }
            Err(_) => {
                // Timeout reached, no more responses
                break
            }
        }
    }
}