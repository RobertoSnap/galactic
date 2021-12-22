use bevy::prelude::*;

use crate::{
    components::{ProjectileBundle, Spaceship},
    constants::LASER_BLUE01,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire.label("input"));
    }
}

pub fn fire(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse: Res<Input<MouseButton>>,
    query: Query<&Transform, With<Spaceship>>,
) {
    let fire = mouse.just_pressed(MouseButton::Left);
    if fire {
        println!("fire");
        let spaceship_transform = query.single();
        let _id = commands
            .spawn_bundle(ProjectileBundle {
                sprite: SpriteBundle {
                    transform: *spaceship_transform,
                    texture: asset_server.load(LASER_BLUE01),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
    }
}
