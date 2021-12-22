use bevy::prelude::*;

use crate::constants::{Constants, LAYERS, LAYER_PLAYER};

#[derive(Component)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn default() -> Self {
        return Self(Vec2::ZERO);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Spaceship {
    pub velocity: Vec2,
    pub movement_speed: f32,
    pub movement_speed_max: f32,
    pub rotation_speed: f32,
    pub rotation_speed_max: f32,
    pub break_power: f32,
}

impl Spaceship {
    pub fn default() -> Self {
        Self {
            velocity: Vec2::new(0., 0.),
            movement_speed: 3.,
            movement_speed_max: 300.,
            rotation_speed: f32::to_radians(90.0), // degrees per second
            rotation_speed_max: f32::to_radians(120.0), // degrees per second
            break_power: 300.,
        }
    }
}
