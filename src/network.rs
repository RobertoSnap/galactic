use renet_udp::{
    client::UdpClient,
    renet::remote_connection::ConnectionConfig,
    server::{ServerEvent, UdpServer},
};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;
use std::{
    env,
    net::{SocketAddr, UdpSocket},
    time::Instant,
};

use bevy::{core::FixedTimestep, prelude::*};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = env::args().collect();
        let is_server = args.iter().find(|arg| **arg == "server").is_some();
        if is_server {
            app.add_startup_system(init_server).add_system_set(
                SystemSet::new()
                    // This prints out "hello world" once every second
                    .with_run_criteria(FixedTimestep::step(30. / 60.))
                    .with_system(server_events),
            );
        } else{
            app.add_startup_system(init).add_system_set(
                SystemSet::new()
                    // This prints out "hello world" once every second
                    .with_run_criteria(FixedTimestep::step(30. / 60.))
                    .with_system(events),
            );
        }
       
    }
}

fn init_server(mut commands: Commands) {
    println!("Running server");
        let server_addr: SocketAddr = format!("127.0.0.1:{}", 5050).parse().unwrap();
        commands.insert_resource(server(server_addr));
}

fn init_client(mut commands: Commands) {


    if is_server {
        println!("Running server");
        let server_addr: SocketAddr = format!("127.0.0.1:{}", 5050).parse().unwrap();
        commands.insert_resource(server(server_addr));
    } else {
      
    }
}

fn server_events(mut commands: Commands, server) {
    server.update(Instant::now() - last_updated).unwrap();
    last_updated = Instant::now();
    received_messages.clear();

    while let Some(event) = server.get_event() {
        match event {
            ServerEvent::ClientConnected(id) => println!("Client {} connected.", id),
            ServerEvent::ClientDisconnected(id, reason) => {
                println!("Client {} disconnected: {}.", id, reason)
            }
        }
    }

    for client_id in server.clients_id().iter() {
        while let Some(message) = server.receive_message(client_id, 0) {
            let text = String::from_utf8(message).unwrap();
            println!("Client {} sent text: {}", client_id, text);
            received_messages.push(text);
        }
    }

    for text in received_messages.iter() {
        server.broadcast_message(0, text.as_bytes().to_vec());
    }

    server.send_packets().unwrap();
}
fn server(addr: SocketAddr) {
    let socket = UdpSocket::bind(addr).unwrap();
    let connection_config = ConnectionConfig::default();
    let mut server: UdpServer = UdpServer::new(64, connection_config, socket).unwrap();
    let mut received_messages = vec![];
    let mut last_updated = Instant::now();
}

fn client(server_addr: SocketAddr) {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = ConnectionConfig::default();
    let mut client = UdpClient::new(socket, server_addr, connection_config).unwrap();
    let stdin_channel = spawn_stdin_channel();

    let mut last_updated = Instant::now();
    loop {
        client.update(Instant::now() - last_updated).unwrap();
        last_updated = Instant::now();
        match stdin_channel.try_recv() {
            Ok(text) => client.send_message(0, text.as_bytes().to_vec()).unwrap(),
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        while let Some(text) = client.receive_message(0) {
            let text = String::from_utf8(text).unwrap();
            println!("Message from server: {}", text);
        }

        client.send_packets().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
