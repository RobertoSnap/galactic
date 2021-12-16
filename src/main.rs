use bevy::prelude::*;
use spawner::SpawnerPlugin;
mod spawner;

pub struct PlayerState {
    spaceship: Option<Entity>,
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(SpawnerPlugin)
        .run();
}

pub struct Materials {
    spaceship: Handle<ColorMaterial>,
}

pub fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // set window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(100, 100));

    // add resources
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        spaceship: materials.add(ColorMaterial::color(Color::WHITE)),
    });
    commands.insert_resource(PlayerState { spaceship: None })
}
