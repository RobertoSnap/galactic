use bevy::prelude::*;

use crate::{
    components::prelude::*,
    constants::LASER_BLUE01,
    event::{EntityAccelerate, EntityDecelerate, EntityRotate},
    game_state::GameState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app // systems to run only in the main menu
            .add_system_set(
                SystemSet::on_update(GameState::Play)
                    .with_system(fire.label("input"))
                    .with_system(spaceship.label("input")),
            );
    }
}

fn spaceship(
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, (With<PlayerControlled>, With<Spaceship>)>,
    mut entity_rotate: EventWriter<EntityRotate>,
    mut entity_accelerate: EventWriter<EntityAccelerate>,
    mut entity_decelerate: EventWriter<EntityDecelerate>,
) {
    for entity in query.iter() {
        let up = keys.pressed(KeyCode::W);
        let down = keys.pressed(KeyCode::S);
        let right = keys.pressed(KeyCode::D);
        let left = keys.pressed(KeyCode::A);

        if up {
            entity_accelerate.send(EntityAccelerate {
                entity: entity,
                acceleration: 1.,
            });
        }
        if down {
            entity_decelerate.send(EntityDecelerate {
                entity: entity,
                deceleration: 1.,
            });
        }
        if left {
            entity_rotate.send(EntityRotate {
                entity: entity,
                rotation: 1.,
            });
        }
        if right {
            entity_rotate.send(EntityRotate {
                entity: entity,
                rotation: -1.,
            });
        }
    }
}

pub fn fire(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse: Res<Input<MouseButton>>,
    query: Query<(&Transform, &Velocity), With<Spaceship>>,
) {
    let fire = mouse.just_pressed(MouseButton::Left);
    if fire {
        println!("fire");
        let (spaceship_transform, spaceship_velocity) = query.single();
        let _id = commands
            .spawn_bundle(ProjectileBundle {
                sprite: SpriteBundle {
                    transform: *spaceship_transform,
                    texture: asset_server.load(LASER_BLUE01),
                    ..Default::default()
                },
                velocity: *spaceship_velocity,
                ..Default::default()
            })
            .id();
    }
}
