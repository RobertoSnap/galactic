use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use galactic_core::*;

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::Index;
use kayak_ui::core::{render, rsx, widget};
use kayak_ui::widgets::{App as UiApp, Window};

mod camera;

pub enum GameStage {
    MENU,
    PLAYING,
}

impl Default for GameStage {
    fn default() -> Self {
        Self::MENU
    }
}

#[widget]
fn CustomWidget() {
    rsx! {
        <>
            <Window position={(50.0, 50.0)} size={(300.0, 300.0)} title={"Window 1".to_string()}>
                {}
            </Window>
            <Window position={(550.0, 50.0)} size={(200.0, 200.0)} title={"Window 2".to_string()}>
                {}
            </Window>
        </>
    }
}
fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    font_mapping.add(asset_server.load("roboto.kayak_font"));

    let context = BevyContext::new(|context| {
        render! {
            <UiApp>
                <CustomWidget />
            </UiApp>
        }
    });

    commands.insert_resource(context);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Galtic client".to_string(),
            vsync: true,
            resizable: true,
            height: 500.,
            width: 500.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyKayakUIPlugin)
        .add_startup_system(startup)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(event::EventPlugin)
        .add_plugin(network::NetworkPlugin)
        .add_plugin(resource::ResourcePlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(projectile::ProjectilePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(camera::CameraPlugin)
        .run();
}
