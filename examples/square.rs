use rand::Rng;
use std::f32::consts::FRAC_PI_4;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_sparse_tilemap::{
    square::{
        map_chunk_layer::{SquareChunkLayer, SquareChunkSettings},
        map_data::SquareMapData,
        SquareTilemapManager,
    },
    tilemap_builder::{tilemap_layer_builder::TilemapLayer, TilemapBuilder},
};
use bst_map_layer_derive::MapLayer;
use lettuces::cell::Cell;

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
        ))
        .add_systems(Startup, (spawn_map, spawn_tiles).chain())
        .init_resource::<ColorHandles>()
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

/// Change to change the square tile sizes
const TILE_SIZE: f32 = 15.0;

/// Change to change the gap between tiles
const TILE_GAP: f32 = 1.0;

#[derive(Hash, Default, Copy, Clone, Reflect)]
struct TileData(u8, u8);

// Resource to hold our map entity so we can use it in systems
#[derive(Resource)]
pub struct MapEntity(Entity);

#[derive(Resource)]
pub struct SquareMeshHandle(Mesh2d);

#[derive(Resource, Default)]
pub struct ColorHandles(Vec<Handle<ColorMaterial>>);

fn spawn_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let map_size = UVec2::new(5, 5);
    let max_chunk_size = UVec2::new(1, 1);

    let mut tilemap_builder =
        TilemapBuilder::<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData>::new(
            TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size)),
            SquareMapData { max_chunk_size },
            SquareChunkSettings { max_chunk_size },
        );
    tilemap_builder.add_layer(
        TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size)),
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

    let Some(tilemap) = tilemap_builder.spawn_tilemap(&mut commands) else {
        return;
    };
    commands.insert_resource(MapEntity(tilemap));
    commands.insert_resource(SquareMeshHandle(Mesh2d(
        meshes.add(RegularPolygon::new(TILE_SIZE, 4)),
    )));
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(
            (TILE_SIZE * map_size.x as f32) / 2.0,
            (TILE_SIZE * map_size.y as f32) / 2.0,
            1.0,
        )),
    ));
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

fn square_to_world_pos(x: i32, y: i32) -> Vec3 {
    // The TILE_SIZE is actually the circumferential radius. To find the actual length of a side,
    // we need to multiply by the square root of 2.
    let side_length = TILE_SIZE * 2_f32.sqrt();
    let coords = Vec2::new(x as f32, y as f32) * (side_length + TILE_GAP);
    coords.extend(1.)
}

fn spawn_tiles(
    map_entity: Res<MapEntity>,
    square_mesh: ResMut<SquareMeshHandle>,
    mut map: SquareTilemapManager<TileData, MapLayers>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut color_materials: ResMut<ColorHandles>,
) {
    map.set_tilemap_entity(map_entity.0);
    map.set_layer(MapLayers::Base);
    let Ok(dimensions) = map.dimensions() else {
        return;
    };

    for y in 0..dimensions.y as i32 {
        for x in 0..dimensions.x as i32 {
            let color = Color::hsl(360. * x as f32 / y as f32, 0.95, 0.7);
            let handle = materials.add(color);
            color_materials.0.push(handle.clone());
            let entity = commands
                .spawn((
                    square_mesh.0.clone(),
                    MeshMaterial2d(handle),
                    Transform::from_translation(square_to_world_pos(x, y))
                        .with_rotation(Quat::from_rotation_z(FRAC_PI_4)),
                ))
                .id();
            let _ = map.set_tile_entity(Cell { x, y }, entity);
        }
    }
}
