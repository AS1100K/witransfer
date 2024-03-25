use std::net::{IpAddr, SocketAddr, UdpSocket};

pub fn discover(port: u16) {
    let broadcast_addr = SocketAddr::new(IpAddr::V4("255.255.255.255".parse().unwrap()), port);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to socket.");
    socket.set_broadcast(true).expect("Failed to set broadcast option");
    let send_buf = socket.send_to("WiTransfer".as_bytes(), broadcast_addr);

    match send_buf {
        Ok(buf) => println!("{}, setBroadcast = {}", buf, socket.broadcast().unwrap()),
        Err(_) => eprintln!("Error Occurred.")
    }
}