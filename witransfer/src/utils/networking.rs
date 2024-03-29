use local_ip_address;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::{
    collections::BTreeMap,
    thread::{self, sleep},
    time::Duration,
};
use whoami;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    identifier: String,
    device_info: DeviceInfo,
    ip_addr: IpAddr,
    max_threads: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    real_name: String,
    user_name: String,
    device_name: String,
    platform: String,
    distro: String,
}

impl Message {
    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize message")
    }
}

/// Sends broadcast message for visibility.
///
/// `port` is of type `u16` which will determine on which port messages are sent and received.
///
/// # Example
///
/// ```no_run
/// use witransfer::networking::discover;
///
/// let port: u16 = 54321;
/// discover(port)
/// ```
///
/// # Panics
///
/// The function will panic if it encounters error with Socket or sending visibility message.
pub fn discover(port: u16) {
    let message = Message {
        identifier: "WiTransfer".to_string(),
        device_info: DeviceInfo {
            real_name: whoami::realname(),
            user_name: whoami::username(),
            device_name: whoami::devicename(),
            platform: whoami::platform().to_string(),
            distro: whoami::distro(),
        },
        ip_addr: local_ip_address::local_ip().unwrap(),
        max_threads: num_cpus::get(),
    };

    let socket =
        Arc::new(UdpSocket::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind to socket"));
    socket
        .set_broadcast(true)
        .expect("Unable to set broadcast option.");
    let socket_clone = Arc::clone(&socket);
    let (tx, rx) = mpsc::channel();

    let devices: Arc<Mutex<BTreeMap<IpAddr, String>>> = Arc::new(Mutex::new(BTreeMap::new()));
    let devices_clone = Arc::clone(&devices);

    let receive_handle = thread::spawn(move || receive_visibility_message(socket_clone, tx));
    let filter_handle = thread::spawn(move || filter_response(devices_clone, rx));
    let send_handle = thread::spawn(move || send_visibility_message(&socket, port, &message));

    send_handle
        .join()
        .expect("Unable to create thread to send visibility message.");

    receive_handle
        .join()
        .expect("Unable to create thread to receive visibility message.");

    filter_handle
        .join()
        .expect("Unable to create thread to filter message.");

    // TODO: Make user select which device they want to connect with.
}

// Sends visibility message on the broadcast
fn send_visibility_message(socket: &Arc<UdpSocket>, port: u16, message: &Message) {
    let broadcast_addr = SocketAddr::new(IpAddr::V4("255.255.255.255".parse().unwrap()), port);

    // TODO: Optimize and reduce socket message if large

    loop {
        let send_buf = socket.send_to(&message.as_bytes(), broadcast_addr);

        match send_buf {
            Ok(buf) => {
                info!("Sent packet size: {}", buf);
            }
            Err(e) => panic!("{}", e),
        }
        sleep(Duration::from_secs(2));
    }
}

// Receive visibility message and send message to another thread for processing.
fn receive_visibility_message(socket: Arc<UdpSocket>, tx: Sender<Message>) {
    info!("Awaiting for responses");
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(50))) {
        eprintln!("Unable to set read timeout to 50 sec. Err -> {}", e);
        return;
    }

    let mut buf = [0; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, _addr)) => {
                let data = &buf[..n];

                match serde_json::from_slice(data) {
                    Ok(message) => {
                        if let Err(e) = tx.send(message) {
                            eprintln!("Unable to send message to filtering thread. Err -> {}", e)
                        }
                    }
                    Err(e) => {
                        eprintln!("Error deserializing message: {}", e);
                        continue;
                    }
                };
                // Clear the buffer
                buf = [0; 1024];
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break; // Exit the loop on error
            }
        }
    }
}

// Process/filter the message received from other thread
fn filter_response(devices: Arc<Mutex<BTreeMap<IpAddr, String>>>, rx: Receiver<Message>) {
    let my_ip = local_ip_address::local_ip().unwrap();
    // TODO: Remove device if it goes out of scope
    loop {
        match rx.recv() {
            Ok(message) => {
                if message.identifier == "WiTransfer".to_string()
                    && message.ip_addr != my_ip
                    && !devices.lock().unwrap().contains_key(&message.ip_addr)
                {
                    // TODO: Logic for what to show user.
                    devices.lock().unwrap().insert(
                        message.ip_addr,
                        format!(
                            "{} - {}",
                            message.device_info.real_name, message.device_info.device_name
                        ),
                    );

                    // Printing for Debuging purposes.
                    println!("{:#?}", devices.lock().unwrap())
                }
            }
            _ => continue,
        }
    }
}
