use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_fast_tilemap::{FastTileMapPlugin, Map, MapBundleManaged};
use bevy_sparse_tilemap::map::chunk::{Chunk, ChunkSettings};
use bevy_sparse_tilemap::square::map_chunk_layer::{
    SquareChunkLayer, SquareChunkLayerConversionSettings,
};
use bevy_sparse_tilemap::square::map_data::{SquareMapData, SquareMapDataConversionSettings};
use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
use bevy_sparse_tilemap::SparseTilemapPlugin;
use bst_map_layer_derive::MapLayer;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Benchmark Example"),
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
        .add_plugins((SparseTilemapPlugin, FastTileMapPlugin::default()))
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (mouse_controls_camera, spawn_or_update_fast_tilemaps),
        )
        .run();
}

pub static TILE_SIZE: f32 = 16.0;

#[derive(MapLayer, Clone, Copy, Default)]
pub enum MapLayers {
    #[default]
    Main,
    Secondary,
}

// Resource to hold our map entity so we can use it in systems
#[derive(Resource)]
pub struct MapEntity(Entity);

#[derive(Hash, Default, Copy, Clone, Reflect)]
struct TileData(u8, u8);

#[derive(Component, Default, Copy, Clone, Reflect)]
pub struct FastTileMap;

#[derive(Component, Default, Copy, Clone, Reflect)]
pub struct ChunkMapSpawned;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let map_size = UVec2::new(15000, 15000);
    let tilemap_builder = TilemapBuilder::<
        TileData,
        MapLayers,
        SquareChunkLayer<TileData>,
        SquareMapData,
    >::new_tilemap_with_main_layer(
        TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size.clone())),
        SquareMapData {
            conversion_settings: SquareMapDataConversionSettings {
                max_chunk_dimensions: UVec2::new(100, 100),
            },
        },
        ChunkSettings {
            max_chunk_size: UVec2::new(100, 100),
        },
    );

    let chunk_conversion_settings = SquareChunkLayerConversionSettings {
        max_chunk_dimensions: UVec2 { x: 5, y: 5 },
    };

    let Some(tilemap) = tilemap_builder.spawn_tilemap(chunk_conversion_settings, &mut commands)
    else {
        return;
    };
    commands.entity(tilemap).insert(SpatialBundle::default());
    commands.insert_resource(MapEntity(tilemap));
}

fn spawn_or_update_fast_tilemaps(
    chunk_query: Query<
        (
            Entity,
            &Chunk<SquareChunkLayer<TileData>, TileData>,
            Option<&Children>,
            Option<&ChunkMapSpawned>,
        ),
        Changed<Chunk<SquareChunkLayer<TileData>, TileData>>,
    >,
    fast_tile_map_query: Query<&Handle<Map>, With<FastTileMap>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map>>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    'main_loop: for (entity, chunk, children, map_spawned_option) in chunk_query.iter() {
        if let Some(_) = map_spawned_option {
            for child in children.unwrap().iter() {
                if let Ok(map) = fast_tile_map_query.get(*child) {
                    let m = match materials.get_mut(map) {
                        None => {
                            // Map texture is not available
                            warn!("no map: {:?}", map);
                            continue;
                        }
                        Some(x) => x,
                    };

                    let mut m = m.indexer_mut();

                    for y in 0..chunk.get_chunk_dimensions().y {
                        for x in 0..chunk.get_chunk_dimensions().x {
                            let i = rng.gen_range(1..12);
                            m.set(x, y, i);
                        }
                    }
                    continue 'main_loop;
                }
            }
        }

        // Create map with the given dimensions of our chunk
        let map = Map::builder(
            // Map size (tiles)
            uvec2(
                chunk.get_chunk_dimensions().x,
                chunk.get_chunk_dimensions().y,
            ),
            // Tile atlas
            asset_server.load("tiles_16.png"),
            // Tile size (pixels)
            vec2(TILE_SIZE, TILE_SIZE),
        )
        .build_and_set(|_| rng.gen_range(0..15));

        commands
            .entity(entity)
            .insert((
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            chunk.chunk_pos.x() as f32
                                * chunk.get_chunk_dimensions().x as f32
                                * TILE_SIZE,
                            chunk.chunk_pos.y() as f32
                                * chunk.get_chunk_dimensions().y as f32
                                * TILE_SIZE,
                            1.0,
                        ),
                        ..default()
                    },
                    ..default()
                },
                ChunkMapSpawned,
            ))
            .with_children(|parent| {
                let mut map_bundle = MapBundleManaged::new(map, &mut materials);
                map_bundle.transform.translation = Vec3::new(
                    chunk.chunk_pos.x() as f32 * chunk.get_chunk_dimensions().x as f32 * TILE_SIZE,
                    chunk.chunk_pos.y() as f32 * chunk.get_chunk_dimensions().y as f32 * TILE_SIZE,
                    1.0,
                );
                parent
                    .spawn(map_bundle)
                    .insert(Transform::from_translation(Vec3::new(1.0, 1.0, 1.0)))
                    // Have the map manage our mesh so it always has the right size
                    .insert(FastTileMap);
            });
    }
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

/// Use RMB for panning
/// Use scroll wheel for zooming
fn mouse_controls_camera(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(
        &GlobalTransform,
        &mut Transform,
        &Camera,
        &mut OrthographicProjection,
    )>,
) {
    for event in mouse_motion_events.read() {
        if mouse_button.pressed(MouseButton::Left) || mouse_button.pressed(MouseButton::Right) {
            for (_, mut transform, _, _) in camera_query.iter_mut() {
                transform.translation.x -= event.delta.x * transform.scale.x;
                transform.translation.y += event.delta.y * transform.scale.y;
            }
        }
    }

    let mut wheel_y = 0.;
    for event in mouse_wheel_events.read() {
        wheel_y += event.y;
    }

    if wheel_y != 0. {
        for (_, mut transform, _, _ortho) in camera_query.iter_mut() {
            let factor = f32::powf(2., -wheel_y / 2.);
            transform.scale *= vec3(factor, factor, 1.0);
            transform.scale = transform
                .scale
                .max(Vec3::splat(1. / 128.))
                .min(Vec3::splat(128.));
        }
    }
}
