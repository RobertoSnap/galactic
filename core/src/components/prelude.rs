use bevy::prelude::*;

use crate::constants::Constants;

#[derive(Component, Debug)]
pub struct Player {
    pub spawn: Vec3,
    pub name: Name,
    pub spaceship: Option<Entity>,
    pub money: Option<Entity>,
}

// #[derive(Bundle)]
// pub struct PlayerBundle {
//     pub spaceship: Spaceship,

// }

impl Default for Player {
    fn default() -> Self {
        Self {
            spawn: Vec3::new(80., 80., Constants::layer_for("player")),
            name: Name::new("Random"),
            spaceship: None,
            money: None,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);
impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Spaceship {
    pub name: Name,
}

impl Default for Spaceship {
    fn default() -> Self {
        Self {
            name: Name::new("Fighter ship 1"),
        }
    }
}

#[derive(Bundle)]
pub struct SpaceshipBundle {
    pub spaceship: Spaceship,
    pub speed: Speed,
    pub rotation_speed: RotationSpeed,
    pub acceleration: Acceleration,
    pub deceleration: Deceleration,
    pub velocity: Velocity,
    pub player_controlled: PlayerControlled,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for SpaceshipBundle {
    fn default() -> Self {
        Self {
            spaceship: Default::default(),
            speed: Default::default(),
            rotation_speed: Default::default(),
            acceleration: Default::default(),
            deceleration: Default::default(),
            velocity: Default::default(),
            player_controlled: Default::default(),
            sprite: Default::default(),
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
            range: Range(500.),
            projectile: Projectile::default(),
            velocity: Velocity(Vec2::ONE),
            sprite: SpriteBundle {
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct PlayerControlled {}
impl Default for PlayerControlled {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Component)]
pub struct Projectile {
    pub traveled: f32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self { traveled: 0.0 }
    }
}

#[derive(Component)]
pub struct Damage {
    pub kinetic: i32,
}

#[derive(Component)]
pub struct Speed(pub f32);
impl Default for Speed {
    fn default() -> Self {
        Self(10.)
    }
}

#[derive(Component)]
pub struct RotationSpeed(pub f32);
impl Default for RotationSpeed {
    fn default() -> Self {
        Self(f32::to_radians(90.0))
    }
}

#[derive(Component)]
pub struct Acceleration(pub f32);
impl Default for Acceleration {
    fn default() -> Self {
        Self(1.)
    }
}
#[derive(Component)]
pub struct Deceleration(pub f32);
impl Default for Deceleration {
    fn default() -> Self {
        Self(300.)
    }
}
#[derive(Component)]
pub struct Range(pub f32);
impl Default for Range {
    fn default() -> Self {
        Self(500.)
    }
}

#[derive(Component, Debug)]
pub struct Money(pub u64);

impl Default for Money {
    fn default() -> Self {
        Self(0)
    }
}

// Map
#[derive(Component)]
pub struct MapTile {
    pub index: i32,
}
