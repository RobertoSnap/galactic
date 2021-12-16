use bevy::{core::FixedTimestep, prelude::*};

use crate::{Materials, PlayerState};

// use bevy::prelude::*;

pub struct Spaceship;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(spawn_player_spaceship.system()),
        );
    }
}

pub fn spawn_player_spaceship(
    mut commands: Commands,
    materials: Res<Materials>,
    mut player_state: ResMut<PlayerState>,
) {
    if None == player_state.spaceship {
        let id = commands
            .spawn_bundle(SpriteBundle {
                material: materials.spaceship.clone(),
                transform: Transform::from_xyz(1., 2., 0.),
                sprite: Sprite::new(Vec2::new(10., 10.)),
                ..Default::default()
            })
            .insert(Spaceship)
            .id();
        player_state.spaceship = Some(id)
    }
}
