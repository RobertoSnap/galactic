use bevy::prelude::*;
use camera::CameraPlugin;
use constants::WORLD_STAGE;
use input::InputPlugin;
use map::MapPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use resource::ResourcePlugin;

mod camera;
mod components;
mod constants;
mod input;
mod map;
mod movement;
mod player;
mod resource;

const TIME_STEP: f32 = 1. / 60.;

fn main() {
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
        .add_plugins(DefaultPlugins)
        .add_plugin(ResourcePlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
