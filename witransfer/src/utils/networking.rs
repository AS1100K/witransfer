use std::net::{IpAddr, SocketAddr, UdpSocket};
use serde::{Serialize, Deserialize};
use serde_json;
use whoami;
use std::thread::sleep;
use std::time::Duration;
use log::info;

#[derive(Serialize, Deserialize)]
struct Message<'a> {
    identifier: &'a str,
    device_info: DeviceInfo,
    ip_addr: IpAddr,
    max_threads: usize
}

#[derive(Serialize, Deserialize)]
struct DeviceInfo {
    real_name: String,
    user_name: String,
    device_name: String,
    platform: String,
    distro: String,
}

impl<'a> Message<'a> {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize message")
    }
}

/// Sends broadcast message for visibility.
///
/// # Example
///
/// ```rust
/// use witransfer::networking::discover;
/// discover(54321)
/// ```
pub fn discover(port: u16) {
    let message = Message {
        identifier: "WiTransfer",
        device_info: DeviceInfo {
            real_name: whoami::realname(),
            user_name: whoami::username(),
            device_name: whoami::devicename(),
            platform: whoami::platform().to_string(),
            distro: whoami::distro()
        },
        ip_addr: local_ip_address::local_ip().unwrap(),
        max_threads: num_cpus::get()
    };

    loop {
        send_visibility_message(port, &message);
        sleep(Duration::from_secs(2));
    }
}

fn send_visibility_message(port: u16, message: &Message) {
    let broadcast_addr = SocketAddr::new(IpAddr::V4("255.255.255.255".parse().unwrap()), port);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to socket");
    socket.set_broadcast(true).expect("Failed to set broadcast option.");

    let send_buf = socket.send_to(&message.as_bytes(), broadcast_addr);

    match send_buf {
        Ok(buf) => {
            info!("Sent packet size: {}", buf);
        },
        Err(e) => panic!("{}", e)
    }
}
