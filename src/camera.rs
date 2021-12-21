pub struct CameraPlugin;
use crate::components::{MainCamera, Spaceship, Velocity};
use bevy::prelude::*;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(camera_follows_player);
    }
}

fn camera_follows_player(
    mut camera_query: Query<(&mut Transform), (With<MainCamera>, Without<Spaceship>)>,
    player_query: Query<&Transform, With<Spaceship>>,
) {
    let (mut camera_transform) = camera_query.single_mut();
    let player_transform = player_query.get_single();
    if let Ok(t) = player_transform {
        camera_transform.translation = t.translation;
    }
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Velocity::default());
}
