use bevy::{core::FixedTimestep, log, math::Vec3Swizzles, prelude::*};

use crate::{spawner::Spaceship, TIME_STEP};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_spaceship),
        );
    }
}

fn move_spaceship(mut query: Query<(&mut Transform, &mut Spaceship)>, keys: Res<Input<KeyCode>>) {
    for (mut transform, mut spaceship) in query.iter_mut() {
        let mut rotation_factor = 0.0;
        let mut movement_factor = 0.0;
        let mut break_or_reverse_factor = 0.0;

        let up = keys.pressed(KeyCode::W);
        let down = keys.pressed(KeyCode::S);
        let right = keys.pressed(KeyCode::D);
        let left = keys.pressed(KeyCode::A);

        if up {
            movement_factor += 1.0;
        }
        if down {
            break_or_reverse_factor += 1.0;
        }
        if left {
            rotation_factor += 1.0;
        }
        if right {
            rotation_factor -= 1.0;
        };

        let rotation_delta =
            Quat::from_rotation_z(rotation_factor * spaceship.rotation_speed * TIME_STEP);
        transform.rotation *= rotation_delta;

        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = movement_factor * spaceship.movement_speed * TIME_STEP;
        let translation_delta = (movement_direction * movement_distance).xy(); // dont need z
        let translation_delta_with_velocity = translation_delta + spaceship.velocity;

        // Break or go
        if break_or_reverse_factor > 0. {
            let break_power = break_or_reverse_factor * spaceship.break_power * TIME_STEP;
            let break_power_as_percentage = 1. - (break_power / 300.);
            spaceship.velocity *= break_power_as_percentage;
            // TODO - Implement reverse
        }
        println!(
            "spaceship.velocity.length() {}",
            spaceship.velocity.length()
        );
        if spaceship.velocity.length() < 1. && movement_factor == 0.0 {
            spaceship.velocity = Vec2::ZERO;
        }
        if movement_factor > 0.0 || spaceship.velocity.length() > 1. {
            transform.translation.x += translation_delta_with_velocity.x;
            transform.translation.y += translation_delta_with_velocity.y;
        }
        spaceship.velocity += translation_delta;
    }
}
