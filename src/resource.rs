use bevy::prelude::*;

use crate::constants::{Constants, LAYERS};

pub struct Galaxy {
    pub map_tiles: Option<Vec<Entity>>,
    pub size: IVec2,
}

impl Galaxy {
    pub fn default() -> Self {
        Self {
            map_tiles: None,
            size: IVec2::new(32, 32),
        }
    }
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
pub fn setup(mut commands: Commands) {
    commands.insert_resource(Galaxy::default());
}
