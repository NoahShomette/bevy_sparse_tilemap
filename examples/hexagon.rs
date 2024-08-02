use bevy::app::FixedUpdate;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::UVec2;
use bevy::prelude::{
    apply_deferred, default, App, Camera2dBundle, Commands, Entity, IntoSystemConfigs, PluginGroup,
    Reflect, RegularPolygon, Res, ResMut, Resource, Startup, Window, WindowPlugin,
};
use bevy::render::mesh::Mesh;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::{Fixed, Time};
use bevy::transform::components::Transform;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_sparse_tilemap::hex::map_chunk_layer::HexagonChunkSettings;
use bevy_sparse_tilemap::hex::map_data::HexMapData;
use bevy_sparse_tilemap::hex::{
    hex_offset_from_orientation, hex_rotation, HexTilemapBuilder, HexTilemapManager,
};

use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
use bst_map_layer_derive::MapLayer;
use lettuces::cell::Cell;
use lettuces::{Hex, HexLayout, HexOrientation, Vec2, Vec3};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Hexagon Example"),
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
        .add_systems(Startup, (spawn_map, apply_deferred, spawn_tiles).chain())
        .add_systems(FixedUpdate, change_random_tile_color)
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .init_resource::<ColorHandles>()
        .run();
}

/// Change circumference to change the hexagon tiles sizes
const HEXAGON_CIRCUMFERENCE: f32 = 15.0;
/// Change orientation to see the difference between flat and pointy topped hexagons
const HEXAGON_ORIENTATION: HexOrientation = HexOrientation::Pointy;

#[derive(MapLayer, Clone, Copy, Default)]
pub enum MapLayers {
    #[default]
    Base,
    Sparse,
}

#[derive(Hash, Default, Copy, Clone, Reflect)]
struct TileData(u8, u8);

// Resource to hold our map entity so we can use it in systems
#[derive(Resource)]
pub struct MapEntity(Entity);

#[derive(Resource)]
pub struct HexagonMeshHandle(Mesh2dHandle);

#[derive(Resource, Default)]
pub struct ColorHandles(Vec<Handle<ColorMaterial>>);

fn spawn_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let map_size = UVec2::new(25, 10);
    let max_chunk_size = UVec2::new(5, 5);

    let mut tilemap_builder = HexTilemapBuilder::new(
        TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size.clone())),
        HexMapData {
            max_chunk_size: max_chunk_size,
        },
        HexagonChunkSettings {
            orientation: HEXAGON_ORIENTATION,
            max_chunk_size,
        },
    );
    tilemap_builder.add_layer(
        TilemapLayer::new_sparse_empty(map_size.x as usize, map_size.y as usize),
        MapLayers::Sparse,
    );

    let Some(tilemap) = tilemap_builder.spawn_tilemap(&mut commands) else {
        return;
    };
    commands.insert_resource(MapEntity(tilemap));
    commands.insert_resource(HexagonMeshHandle(Mesh2dHandle(
        meshes.add(RegularPolygon::new(HEXAGON_CIRCUMFERENCE, 6)),
    )));

    let mut camerabundle = Camera2dBundle::default();
    camerabundle.transform = Transform::from_translation(Vec3::new(
        (HEXAGON_CIRCUMFERENCE * map_size.x as f32) / 2.0,
        -((HEXAGON_CIRCUMFERENCE * map_size.y as f32) / 2.0),
        1.0,
    ));
    commands.spawn(camerabundle);
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

fn spawn_tiles(
    map_entity: Res<MapEntity>,
    hex_mesh: ResMut<HexagonMeshHandle>,
    mut map: HexTilemapManager<TileData, MapLayers>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut color_materials: ResMut<ColorHandles>,
) {
    map.set_tilemap_entity(map_entity.0);
    map.set_layer(MapLayers::Base);
    let Ok(dimensions) = map.dimensions() else {
        return;
    };

    let hex_layout = HexLayout {
        orientation: HEXAGON_ORIENTATION,
        origin: Vec2::ZERO,
        hex_size: Vec2::splat(HEXAGON_CIRCUMFERENCE),
        invert_x: false,
        invert_y: false,
    };

    for y in 0..dimensions.y as i32 {
        for x in 0..dimensions.x as i32 {
            let axial_coords = Cell::from_offset_coordinates(
                [x, y],
                hex_offset_from_orientation(HEXAGON_ORIENTATION),
            );

            let color = Color::hsl(360. * x as f32 / y as f32, 0.95, 0.7);
            let handle = materials.add(color);
            color_materials.0.push(handle.clone());
            let entity = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: hex_mesh.0.clone(),
                    material: handle,
                    transform: Transform::from_translation(
                        hex_layout
                            .hex_to_world_pos(Hex::new(axial_coords.x, axial_coords.y))
                            .extend(1.0),
                    )
                    .with_rotation(hex_rotation(HEXAGON_ORIENTATION)),
                    ..default()
                })
                .id();
            let _ = map.set_tile_entity(axial_coords, entity);
        }
    }
}

fn change_random_tile_color(
    map_entity: Res<MapEntity>,
    mut map: HexTilemapManager<TileData, MapLayers>,
    mut commands: Commands,
    colors: ResMut<ColorHandles>,
) {
    map.set_tilemap_entity(map_entity.0);
    map.set_layer(MapLayers::Base);
    let Ok(dimensions) = map.dimensions() else {
        return;
    };

    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..dimensions.x as i32);
    let y = rng.gen_range(0..dimensions.y as i32);

    let axial_coords =
        Cell::from_offset_coordinates([x, y], hex_offset_from_orientation(HEXAGON_ORIENTATION));

    let Some(color_handle) = colors.0.get(rng.gen_range(0..colors.0.len())) else {
        return;
    };

    let Ok(entity) = map.get_tile_entity(axial_coords) else {
        return;
    };

    commands.entity(entity).insert(color_handle.clone());
}
