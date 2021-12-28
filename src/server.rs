use std::{
    collections::{HashMap, HashSet},
    net::{SocketAddr, UdpSocket},
    thread,
    time::Duration,
};

use rand::Error;
use renet::error::DisconnectionReason;
use renet_udp::{
    renet::{error::RenetError, remote_connection::ConnectionConfig},
    server::ServerEvent,
    server::UdpServer,
};

#[derive(Debug, Serialize, Deserialize)]
enum ClientMessages {
    Text(u64, String),
    Init { nick: String },
}

#[derive(Debug, Serialize, Deserialize)]
enum ServerMessages {
    ClientConnected(SocketAddr, String),
    ClientDisconnected(SocketAddr, DisconnectionReason),
    ClientMessage(SocketAddr, String),
    MessageReceived(u64),
    InitClient {
        clients: HashMap<SocketAddr, String>,
    },
}

use bincode::Options;
use log::info;
use serde::{Deserialize, Serialize};

fn serve() -> Result<(), Into<Error>> {
    let socket = UdpSocket::bind("127.0.0.0:5000").unwrap();
    let server_config = ServerConfig::default();
    // Create one channel for each type reliable (0), unreliable(1), block(2)
    let connection_config = ConnectionConfig::default();
    let mut server: UdpServer = UdpServer::new(server_config, connection_config, socket)?;

    let frame_duration = Duration::from_millis(100);
    loop {
        server.update(frame_duration)?;
        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected(id) => println!("Client {} connected.", id),
                ServerEvent::ClientDisconnected(id, reason) => {
                    println!("Client {} disconnected: {}", id, reason)
                }
            }
        }

        for client_id in server.clients_id().iter() {
            while let Some(message) = server.receive_message(client_id, 0) {
                let text = String::from_utf8(message)?;
                println!("Client {} sent text: {}", client_id, text);
                server.broadcast_message(0, text.as_bytes().to_vec());
            }
        }

        server.send_packets()?;
        thread::sleep(frame_duration);
    }
    Ok(())
}
