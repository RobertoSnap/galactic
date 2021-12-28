use bevy::prelude::*;

use crate::{
    components::MapTile,
    constants::{LAYER_MAP, SPACE_BG_3},
    resource::Galaxy,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, render);
    }
}
fn render(mut commands: Commands, asset_server: Res<AssetServer>, galaxy: Res<Galaxy>) {
    let mut index = 0;
    for x in 0..=galaxy.size.x {
        for y in 0..=galaxy.size.y {
            index += 1;
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(64. * x as f32, 64. * y as f32, LAYER_MAP),
                        rotation: Quat::from_rotation_y(0.),
                        scale: Vec3::new(1., 1., 1.),
                    },
                    texture: asset_server.load(SPACE_BG_3),
                    ..Default::default()
                })
                .insert(MapTile { index: index })
                .id();
        }
    }
}
