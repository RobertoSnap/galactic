use bevy::prelude::*;
use movement::MovementPlugin;
use spawner::SpawnerPlugin;
mod constants;
mod movement;
mod spawner;

const TIME_STEP: f32 = 1. / 60.;
// const TIMESTEP_2_PER_SECOND: f64 = 30. / 60.;

pub struct PlayerState {
    spaceship: Option<Entity>,
}

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
        .add_startup_system(setup)
        .add_plugin(SpawnerPlugin)
        .add_plugin(MovementPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

pub fn setup(mut commands: Commands) {
    // set window
    // let window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(100, 100));

    // add resources
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(PlayerState { spaceship: None })
}
