use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::UVec2;
use bevy::prelude::{default, App, PluginGroup, Reflect, Startup, Window, WindowPlugin, Resource, Entity, Commands};
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sparse_tilemap::map::chunk::ChunkSettings;
use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
use bst_map_layer_derive::MapLayer;
use rand::Rng;
use bevy_sparse_tilemap::tilemap_manager::TilemapManager;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Simple Example"),
                resolution: (1270.0, 720.0).into(),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, spawn_map)
        .run();
}

#[derive(MapLayer, Clone, Copy, Default)]
pub enum MapLayers {
    #[default]
    Base,
    Sparse,
    SparseTwo,
    SparseThree,
    DenseExtra,
}

#[derive(Hash, Default, Copy, Clone, Reflect)]
struct TileData(u8, u8);

// Resource to hold our map entity so we can use it in systems
#[derive(Resource)]
pub struct MapEntity(Entity);

fn spawn_map(mut tilemap_builder: TilemapBuilder<TileData, MapLayers>, mut commands: Commands) {
    let map_size = UVec2::new(500, 500);
    tilemap_builder.new_tilemap_with_main_layer(
        TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size.clone())),
        ChunkSettings {
            max_chunk_size: UVec2::new(100, 100),
        },
    );
    tilemap_builder.add_layer(
        TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size.clone())),
        MapLayers::DenseExtra,
    );
    tilemap_builder.add_layer(
        TilemapLayer::new_sparse_empty(map_size.x as usize, map_size.y as usize),
        MapLayers::Sparse,
    );
    tilemap_builder.add_layer(
        TilemapLayer::new_sparse_empty(map_size.x as usize, map_size.y as usize),
        MapLayers::SparseTwo,
    );
    tilemap_builder.add_layer(
        TilemapLayer::new_sparse_empty(map_size.x as usize, map_size.y as usize),
        MapLayers::SparseThree,
    );
    let tilemap = tilemap_builder.spawn_tilemap();
    commands.insert_resource(MapEntity(tilemap));

}

fn generate_random_tile_data(size_to_generate: UVec2) -> Vec<Vec<TileData>> {
    let mut rng = rand::thread_rng();

    let mut vec: Vec<Vec<TileData>> = vec![];
    for _ in 0..size_to_generate.y as usize {
        let mut x_vec: Vec<TileData> = vec![];
        for _ in 0..size_to_generate.x as usize {
            let zero = rng.gen_range(1..12);
            let one = rng.gen_range(1..12);

            x_vec.push(TileData(zero, one));
        }
        vec.push(x_vec);
    }
    vec
}
