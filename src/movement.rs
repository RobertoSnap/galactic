use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*};

use crate::{spawner::Spaceship, TIMESTEP_60_PER_SECOND};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(1. / 60.))
                .with_system(move_spaceship),
        );
    }
}

fn move_spaceship(mut query: Query<(&mut Transform, &mut Spaceship)>, keys: Res<Input<KeyCode>>) {
    for (mut transform, mut spaceship) in query.iter_mut() {
        let mut trust_direction = Vec2::ZERO;
        let speed = 1.;

        let up = keys.pressed(KeyCode::W);
        let down = keys.pressed(KeyCode::S);
        let right = keys.pressed(KeyCode::D);
        let left = keys.pressed(KeyCode::A);

        if up {
            trust_direction.y += speed;
        }
        if down {
            trust_direction.y -= speed;
        }
        if right {
            trust_direction.x += speed;
        }
        if left {
            trust_direction.x -= speed;
        }
        // if trust_direction == Vec2::ZERO && spaceship.velocity.x != 0. && spaceship.velocity.y != 0.
        // {
        //     println!("Should break");
        //     // spaceship.velocity = spaceship.velocity.mul(2.);
        //     transform.scale = Vec3::new(0.8, 0.8, 0.8);
        // } else {
        //     transform.scale = Vec3::new(0.5, 0.5, 0.);
        //     println!("trust_direction before {}", trust_direction);
        //     trust_direction = trust_direction * TIMESTEP_60_PER_SECOND as f32;
        //     println!("trust_direction after {}", trust_direction);
        //     spaceship.velocity = spaceship.velocity * trust_direction;
        // }

        println!("trust_direction before {}", trust_direction);
        trust_direction = trust_direction * TIMESTEP_60_PER_SECOND as f32;
        println!("trust_direction after {}", trust_direction);

        spaceship.velocity += trust_direction;
        println!("spaceship.velocity {}", spaceship.velocity.abs());

        println!("transform.translation {}", transform.translation.xy().abs());

        if spaceship.velocity.x.abs() < 0.2 && spaceship.velocity.y.abs() < 0.2 {
            println!("Clamped");
        } else {
            transform.translation += Vec3::new(spaceship.velocity.x, spaceship.velocity.y, 0.);
        }

        // let spaceship_pos = transform.translation.truncate();
        // let target = spaceship_pos * spaceship.velocity;
        // let angle = transform.rotation.
        // println!("Velocity {:?}", spaceship.velocity);
        // println!("spaceship_pos {:?}", spaceship_pos);
        // println!("Target {:?}", target);
        // println!("Angle {}", angle);
        // transform.rotation = Quat::from_rotation_z(angle)
        let enemy_forward = (transform.rotation * Vec3::Y).xy();
        let to_player = transform.translation.truncate() * spaceship.velocity;
        let forward_dot_player = enemy_forward.dot(to_player);
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }
        let enemy_right = (transform.rotation * Vec3::X).xy();
        let right_dot_player = enemy_right.dot(to_player);
        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); // clamp acos for safety
        let rotation_angle = rotation_sign
            * (spaceship.rotation_speed * TIMESTEP_60_PER_SECOND as f32).min(max_angle);
        let rotation_delta = Quat::from_rotation_z(rotation_angle);
        transform.rotation *= rotation_delta;
    }
}
