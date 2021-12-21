use bevy::prelude::*;

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
