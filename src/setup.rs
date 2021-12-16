use crate::prelude::*;

pub struct Materials {
    spaceship: Handle<ColorMaterial>,
}

pub fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // add resources
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        spaceship: materials.add(ColorMaterial::color(Color::WHITE)),
    });
    // set window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(100, 100));
}
