use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use movement::MovementPlugin;
use spawner::SpawnerPlugin;

mod constants;
mod movement;
mod spawner;

const TIME_STEP: f32 = 1. / 60.;
// const TIMESTEP_2_PER_SECOND: f64 = 30. / 60.;

pub struct PlayerState {
    spaceship: Option<Entity>,
}

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
        .add_startup_system(setup)
        .add_plugin(SpawnerPlugin)
        .add_plugin(MovementPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // set window
    // let window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(100, 100));

    // add resources
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(PlayerState { spaceship: None });

    // Map
    let texture_handle = asset_server.load("Space/space_bg_3.png");
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    let layer_settings = LayerSettings::new(
        MapSize(10, 10),
        ChunkSize(64, 64),
        TileSize(16.0, 16.0),
        TextureSize(96.0, 16.0),
    );
    let center = layer_settings.get_pixel_center();
    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);
    // map.add_layer(&mut commands, 0u16, layer_entity);
    // layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
    //     // True here refers to tile visibility.
    //     *tile_data = Some(TileBundle::default());
    //     // Tile entity might not exist at this point so you'll need to create it.
    //     if tile_entity.is_none() {
    //         *tile_entity = Some(commands.spawn().id());
    //     }
    //     commands
    //         .entity(tile_entity.unwrap())
    //         .insert(LastUpdate::default());
    // });

    // map_query.build_layer(&mut commands, layer_builder, texture_handle);

    // // Spawn Map
    // // Required in order to use map_query to retrieve layers/tiles.
    // commands
    //     .entity(map_entity)
    //     .insert(map)
    //     .insert(Transform::from_xyz(-center.x, -center.y, 0.0))
    //     .insert(GlobalTransform::default());
}

pub fn map(mut commands: Commands) {
    todo!()
}
