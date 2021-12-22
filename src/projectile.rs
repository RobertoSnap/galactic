use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    components::{Projectile, Speed, Velocity},
    TIME_STEP,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_projectile.after("input"));
    }
}

fn move_projectile(mut query: Query<(&mut Transform, &Velocity, &Speed), With<Projectile>>) {
    for (mut transform, velocity, speed) in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = speed.0 * TIME_STEP;
        let translation_delta = (movement_direction * movement_distance).xy(); // do
        println!("{}", translation_delta);
        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y;
    }
}
