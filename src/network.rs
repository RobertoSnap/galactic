use renet_udp::{
    client::UdpClient,
    renet::remote_connection::ConnectionConfig,
    server::{ServerEvent, UdpServer},
};
use std::{
    env,
    net::{SocketAddr, UdpSocket},
    time::Instant,
};

use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};

use crate::event::SpawnPlayer;

pub enum MultiplayerKind {
    Server(UdpServer),
    Client(UdpClient),
}

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
        } else {
            app.add_startup_system(init_client).add_system_set(
                SystemSet::new()
                    // This prints out "hello world" once every second
                    .with_run_criteria(FixedTimestep::step(30. / 60.))
                    .with_system(some_client),
            );
        }
    }
}

pub fn run_if_server(mode: Res<MultiplayerKind>) -> ShouldRun {
    match mode.as_ref() {
        MultiplayerKind::Server(_) => ShouldRun::Yes,
        MultiplayerKind::Client(_) => ShouldRun::No,
    }
}

pub fn run_if_client(mode: Res<MultiplayerKind>) -> ShouldRun {
    match mode.as_ref() {
        MultiplayerKind::Server(_) => ShouldRun::No,
        MultiplayerKind::Client(_) => ShouldRun::Yes,
    }
}

fn init_server(mut commands: Commands) {
    println!("Running server");
    let server_addr: SocketAddr = format!("127.0.0.1:{}", 5050).parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = ConnectionConfig::default();
    let server: UdpServer = UdpServer::new(64, connection_config, socket).unwrap();
    commands.insert_resource(MultiplayerKind::Server(server));
}

fn init_client(mut commands: Commands, mut event_writer: EventWriter<SpawnPlayer>) {
    println!("Running client");
    let server_addr: SocketAddr = format!("127.0.0.1:{}", 5050).parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = ConnectionConfig::default();
    let client = UdpClient::new(socket, server_addr, connection_config).unwrap();
    commands.insert_resource(MultiplayerKind::Client(client));
    event_writer.send(SpawnPlayer {
        name: "Random".to_string(),
    });
}

fn server_events(mut mp: ResMut<MultiplayerKind>) {
    let last_updated = Instant::now();
    let mut received_messages = vec![];
    let server = match mp.as_mut() {
        MultiplayerKind::Server(s) => s,
        _ => todo!(),
    };
    server.update(Instant::now() - last_updated).unwrap();
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

    for mut text in received_messages.iter() {
        let mut new_string = text.clone();
        new_string.push_str(" Pong");
        server.broadcast_message(0, new_string.as_bytes().to_vec());
    }

    server.send_packets().unwrap();
}

fn some_client(mut mp: ResMut<MultiplayerKind>) {
    if let MultiplayerKind::Client(client) = mp.as_mut() {
        let last_updated = Instant::now();
        client.update(Instant::now() - last_updated).unwrap();
        client.send_message(0, "Ping".as_bytes().to_vec()).unwrap();

        while let Some(text) = client.receive_message(0) {
            let text = String::from_utf8(text).unwrap();
            println!("Message from server: {}", text);
        }

        client.send_packets().unwrap();
    }
}
