use bevy::{core::FixedTimestep, prelude::*};

use crate::{
    components::Spaceship,
    constants::{LAYER_PLAYER, PLAYER_SHIP1_BLUE},
    resource::Player,
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(spawn_player_spaceship),
        );
    }
}

pub fn spawn_player_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
) {
    if None == player.spaceship {
        let id = commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(80., 80., LAYER_PLAYER),
                    rotation: Quat::from_rotation_y(0.),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                },
                texture: asset_server.load(PLAYER_SHIP1_BLUE),
                ..Default::default()
            })
            .insert(Spaceship {
                velocity: Vec2::new(0., 0.),
                movement_speed: 3.,
                movement_speed_max: 300.,
                rotation_speed: f32::to_radians(90.0), // degrees per second
                rotation_speed_max: f32::to_radians(120.0), // degrees per second
                break_power: 300.,
            })
            .id();
        player.spaceship = Some(id)
    }
}
