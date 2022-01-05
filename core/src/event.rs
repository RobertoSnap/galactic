use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityRotate>()
            .add_event::<EntityAccelerate>()
            .add_event::<EntityDecelerate>()
            .add_event::<SpawnPlayer>();
    }
}

pub struct SpawnPlayer {
    pub name: String,
}

pub struct EntityRotate {
    pub entity: Entity,
    pub rotation: f32,
}

pub struct EntityAccelerate {
    pub entity: Entity,
    pub acceleration: f32,
}

pub struct EntityDecelerate {
    pub entity: Entity,
    pub deceleration: f32,
}
