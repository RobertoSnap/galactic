use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use galactic_core;

use kayak_ui::bevy::BevyKayakUIPlugin;

mod camera;
mod main_menu;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Galtic client".to_string(),
            vsync: true,
            resizable: true,
            height: 500.,
            width: 500.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_state(galactic_core::game_state::GameState::MainMenu)
        .add_plugin(BevyKayakUIPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(galactic_core::game_state::GameStatePlugin)
        .add_plugin(galactic_core::event::EventPlugin)
        .add_plugin(galactic_core::network::NetworkPlugin)
        .add_plugin(galactic_core::resource::ResourcePlugin)
        .add_plugin(galactic_core::map::MapPlugin)
        .add_plugin(galactic_core::movement::MovementPlugin)
        .add_plugin(galactic_core::projectile::ProjectilePlugin)
        .add_plugin(galactic_core::input::InputPlugin)
        .add_plugin(galactic_core::player::PlayerPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .run();
}
