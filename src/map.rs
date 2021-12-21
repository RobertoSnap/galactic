use bevy::prelude::*;

use crate::{
    constants::{LAYER_MAP, SPACE_BG_3},
    resource::WorldState,
};

pub struct Map {
    x: i32,
    y: i32,
}
#[derive(Component)]
pub struct MapTile {
    index: i32,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render);
    }
}
fn render(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_state: ResMut<WorldState>,
) {
    let map = Map { x: 20, y: 3 };
    if None == world_state.map {
        let mut index = 0;
        for x in 0..=map.x {
            for y in 0..=map.y {
                index += 1;
                let tile = commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(64. * x as f32, 64. * y as f32, LAYER_MAP),
                            rotation: Quat::from_rotation_y(0.),
                            scale: Vec3::new(1., 1., 1.),
                        },
                        texture: asset_server.load(SPACE_BG_3),
                        ..Default::default()
                    })
                    .insert(MapTile { index })
                    .id();
            }
        }
    }
}
