use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    components::{Projectile, Range, Speed, Velocity},
    TIME_STEP,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_projectile.after("input"));
    }
}

fn move_projectile(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Velocity,
            &Speed,
            &mut Projectile,
            &Range,
        ),
        With<Projectile>,
    >,
) {
    for (entity, mut transform, velocity, speed, mut projectile, range) in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = speed.0 * TIME_STEP;
        projectile.traveled += movement_distance;
        if projectile.traveled >= range.0 {
            commands.entity(entity).despawn();
        }
        let translation_delta = (movement_direction * movement_distance).xy(); // do
        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y;
    }
}
