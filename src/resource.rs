use bevy::prelude::*;

pub struct Player {
    pub spaceship: Option<Entity>,
}
pub struct WorldState {
    pub map: Option<Entity>,
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
pub fn setup(mut commands: Commands) {
    commands.insert_resource(Player { spaceship: None });
    commands.insert_resource(WorldState { map: None });
}
