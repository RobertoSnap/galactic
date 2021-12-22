use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn default() -> Self {
        return Self(Vec2::ONE);
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

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub speed: Speed,
    pub range: Range,
    pub damage: Damage,
    pub velocity: Velocity,
    pub projectile: Projectile,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            speed: Speed(300.),
            damage: Damage { kinetic: 10 },
            range: Range(50.),
            projectile: Projectile {},
            velocity: Velocity(Vec2::ONE),
            sprite: SpriteBundle {
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Damage {
    pub kinetic: i32,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Range(pub f32);
