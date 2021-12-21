use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*};
use bevy_remote_devtools_plugin::RemoteDevToolsPlugin;
use camera::CameraPlugin;
use map::MapPlugin;
use movement::MovementPlugin;
use resource::ResourcePlugin;
use spawner::SpawnerPlugin;

mod camera;
mod components;
mod constants;
mod map;
mod movement;
mod resource;
mod spawner;

const TIME_STEP: f32 = 1. / 60.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Galtic".to_string(),
            vsync: true,
            resizable: true,
            height: 500.,
            width: 500.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RemoteDevToolsPlugin::new("Galatic debug", 3030))
        // Optional: If you want to see fps and frame time in the tools.
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // RemoteDevToolsPlugin will replace bevys LogPlugin with a similar implementation.
        // LogSettings Resource can be still used to configure what logs are shown.
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_plugin(ResourcePlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugin(MovementPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
