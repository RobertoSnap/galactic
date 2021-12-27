use bevy::prelude::*;

use crate::{
    components::{Money, Player, Spaceship, SpaceshipBundle},
    constants::{Constants, PLAYER_SHIP1_BLUE},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn)
            .add_system(spawn_spaceship);
    }
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert(Player {
        money: None,
        name: Name::new("Player A"),
        spaceship: None,
        spawn: Default::default(),
    });
}
pub fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Player>,
) {
    for mut player in query.iter_mut() {
        if player.spaceship == None {
            let spaceship_id = commands
                .spawn_bundle(SpaceshipBundle {
                    sprite: SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(80., 80., Constants::layer_for("player")),
                            rotation: Quat::from_rotation_y(0.),
                            scale: Vec3::new(0.3, 0.3, 0.3),
                        },
                        texture: asset_server.load(PLAYER_SHIP1_BLUE),
                        sprite: Default::default(),
                        global_transform: Default::default(),
                        visibility: Default::default(),
                        computed_visibility: Default::default(),
                    },
                    acceleration: Default::default(),
                    spaceship: Default::default(),
                    speed: Default::default(),
                    rotation_speed: Default::default(),
                    deceleration: Default::default(),
                    velocity: Default::default(),
                    player_controlled: Default::default(),
                })
                .insert(Spaceship::default())
                .id();
            player.spaceship = Some(spaceship_id)
        }
    }
}
