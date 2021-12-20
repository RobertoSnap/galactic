use bevy::{core::FixedTimestep, prelude::*};

use crate::{constants::PLAYER_SHIP1_BLUE, PlayerState};

// use bevy::prelude::*;

#[derive(Component)]
pub struct Spaceship {
    pub velocity: Vec2,
    pub speed: i32,
    pub speed_max: i32,
    pub rotation_speed: f32,
    pub rotation_speed_max: f32,
}

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
    mut player_state: ResMut<PlayerState>,
) {
    if None == player_state.spaceship {
        let id = commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(50., 50., 0.),
                    rotation: Quat::from_rotation_y(0.),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                },
                texture: asset_server.load(PLAYER_SHIP1_BLUE),
                // sprite: Sprite {
                //     ..Default::default()
                // },
                ..Default::default()
            })
            .insert(Spaceship {
                velocity: Vec2::ZERO,
                speed: 10,
                speed_max: 50,
                rotation_speed: f32::to_radians(240.0), // degrees per second
                rotation_speed_max: f32::to_radians(360.0), // degrees per second
            })
            .id();
        player_state.spaceship = Some(id)
    }
}
