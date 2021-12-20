use std::f32::consts::PI;

use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*};

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

        let up = keys.pressed(KeyCode::W);
        let down = keys.pressed(KeyCode::S);
        let right = keys.pressed(KeyCode::D);
        let left = keys.pressed(KeyCode::A);

        if up {
            movement_factor += 1.0;
        }
        if down {
            movement_factor -= 1.0;
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
        println!("movement_distance {}", movement_distance);
        let translation_delta = (movement_direction * movement_distance).xy(); // dont need z
        let translation_delta_with_velocity = translation_delta + spaceship.velocity;

        // Break

        if spaceship.velocity.x.abs() < 0.5 && spaceship.velocity.y.abs() < 0.5 {
            println!("Clamped");
        } else {
            transform.translation += Vec3::new(
                translation_delta_with_velocity.x,
                translation_delta_with_velocity.y,
                0.,
            );
        }
        spaceship.velocity += translation_delta;

        // println!("rot after {}", forward);
        // let angular_velocity = ((forward - spaceship.velocity) / TIME_STEP) * (linear_trust);
        // println!("angular_velocity {}", angular_velocity);

        // transform.translation += Vec3::new(angular_velocity.x, angular_velocity.y, 0.);
        // println!("trust_direction before {}", trust_direction);
        // trust_direction = trust_direction * TIMESTEP_60_PER_SECOND as f32;
        // println!("trust_direction after {}", trust_direction);

        // spaceship.velocity += trust_direction;
        // println!("spaceship.velocity {}", spaceship.velocity.abs());

        // println!("transform.translation {}", transform.translation.xy().abs());

        // if spaceship.velocity.x.abs() < 0.2 && spaceship.velocity.y.abs() < 0.2 {
        //     println!("Clamped");
        // } else {
        //     transform.translation += Vec3::new(spaceship.velocity.x, spaceship.velocity.y, 0.);
        // }
    }
}
