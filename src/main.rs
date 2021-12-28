use std::{
    env,
    fmt::Result,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    reflect::erased_serde::private::serde::de::value::Error,
};
use camera::CameraPlugin;

use event::EventPlugin;
use input::InputPlugin;
use map::MapPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use resource::ResourcePlugin;

mod camera;
mod components;
mod constants;
mod event;
mod input;
mod map;
mod movement;
mod player;
mod projectile;
mod resource;
// mod server;

const TIME_STEP: f32 = 1. / 60.;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let is_server = args.iter().find(|arg| **arg == "server").is_some();

    if is_server {
        println!("is server");
        // let socket = UdpSocket::bind("127.0.0.0:5000").unwrap();
        let mut server = server::ChatServer::new(SocketAddr::V4(SocketAddr::));
        server.update(Duration::from_secs(1));
    }
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Galtic".to_string(),
            vsync: true,
            resizable: true,
            height: 500.,
            width: 500.,
            ..Default::default()
        })
        .add_plugin(EventPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ResourcePlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
