use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use galactic_core::*;

mod camera;
mod ui;

pub enum GameStage {
    MENU,
    PLAYING,
}

impl Default for GameStage {
    fn default() -> Self {
        Self::MENU
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Galtic client".to_string(),
            vsync: true,
            resizable: true,
            height: 500.,
            width: 500.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(event::EventPlugin)
        .add_plugin(network::NetworkPlugin)
        .add_plugin(resource::ResourcePlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(projectile::ProjectilePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(ui::UIPlugin)
        .run();
}
