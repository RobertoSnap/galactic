use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .run();
}
