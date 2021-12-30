use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    components::prelude::*,
    event::{EntityAccelerate, EntityDecelerate, EntityRotate},
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rotate_spaceship.after("input"));
        app.add_system(accelerate_spaceship.after("input"));
        app.add_system(move_spaceship.after("input"));
        app.add_system(decelerate_spaceship.after("input"));
    }
}

fn rotate_spaceship(
    time: Res<Time>,
    mut events: EventReader<EntityRotate>,
    mut query: Query<(Entity, &mut Transform, &RotationSpeed), With<Spaceship>>,
) {
    events.iter().for_each(|event| {
        let (_, mut transform, rotation_speed) =
            query.get_mut(event.entity).expect("entity from event.");
        let rotation_delta =
            Quat::from_rotation_z(event.rotation * rotation_speed.0 * time.delta_seconds());
        transform.rotation *= rotation_delta;
    });
}

fn move_spaceship(mut query: Query<(&mut Transform, &Velocity), With<Spaceship>>) {
    query.iter_mut().for_each(|(mut transform, velocity)| {
        // Clamp to stop when not trusting and movement close to 0
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    });
}

fn accelerate_spaceship(
    time: Res<Time>,
    mut events: EventReader<EntityAccelerate>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Acceleration), With<Spaceship>>,
) {
    events.iter().for_each(|event| {
        let (_, transform, mut velocity, acceleration) =
            query.get_mut(event.entity).expect("entity in event");
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = event.acceleration * acceleration.0 * time.delta_seconds();
        let translation_delta = (movement_direction * movement_distance).xy(); // dont need z
        velocity.0 += translation_delta;
    });
}

fn decelerate_spaceship(
    time: Res<Time>,
    mut events: EventReader<EntityDecelerate>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Deceleration), With<Spaceship>>,
) {
    events.iter().for_each(|event| {
        let (_, _, mut velocity, deceleration) =
            query.get_mut(event.entity).expect("entity in event");
        // Clamp to stop when close to 0
        if velocity.0.length() < 0.5 {
            velocity.0 = Vec2::ZERO;
        } else {
            let break_power = event.deceleration * deceleration.0 * time.delta_seconds();
            let break_power_as_percentage = 1. - (break_power / 300.);
            velocity.0 *= break_power_as_percentage;
        }
    });
    // TODO _ implement reverse
}
