use bevy::prelude::*;

use crate::{components::Spaceship, constants::PLAYER_SHIP1_BLUE, resource::Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_spaceship);
    }
}

pub fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
) {
    if None == player.spaceship {
        // TODO
        let spaceship_id = commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: player.default_spawn,
                    rotation: Quat::from_rotation_y(0.),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                },
                texture: asset_server.load(PLAYER_SHIP1_BLUE),
                ..Default::default()
            })
            .insert(Spaceship::default())
            .id();
        player.spaceship = Some(spaceship_id)
    }
}
