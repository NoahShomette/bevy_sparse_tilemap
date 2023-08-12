use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_fast_tilemap::{Map, MapBundle, MeshManagedByMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sparse_tilemap::map::chunk::{Chunk, ChunkSettings};
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
            WorldInspectorPlugin::default(),
        ))
        .add_plugins(SparseTilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (mouse_controls_camera, spawn_or_update_fast_tilemaps),
        )
        .run();
}

#[derive(MapLayer, Clone, Copy, Default)]
pub enum MapLayers {
    #[default]
    Main,
    Secondary,
}

// Resource to hold our map entity so we can use it in systems
#[derive(Resource)]
pub struct MapEntity(Entity);

#[derive(Default, Copy, Clone, Reflect)]
struct TileData(u8, u8);

#[derive(Component, Default, Copy, Clone, Reflect)]
pub struct FastTileMap;

#[derive(Component, Default, Copy, Clone, Reflect)]
pub struct ChunkMapSpawned;

fn startup(mut tilemap_builder: TilemapBuilder<TileData, MapLayers>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let map_size = UVec2::new(500, 500);
    tilemap_builder.new_tilemap_with_main_layer(
        TilemapLayer::new_dense_from_vecs(generate_random_tile_data(map_size.clone())),
        ChunkSettings {
            max_chunk_size: UVec2::new(100, 100),
        },
    );
    let tilemap = tilemap_builder.spawn_tilemap();
    commands.entity(tilemap).insert(SpatialBundle::default());
    commands.insert_resource(MapEntity(tilemap));
}

fn spawn_or_update_fast_tilemaps(
    chunk_query: Query<
        (
            Entity,
            &Chunk<TileData>,
            Option<&Children>,
            Option<&ChunkMapSpawned>,
        ),
        Changed<Chunk<TileData>>,
    >,
    fast_tile_map_query: Query<&mut Map, With<FastTileMap>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    'main_loop: for (entity, chunk, children, map_spawned_option) in chunk_query.iter() {
        if let Some(_) = map_spawned_option {
            for child in children.unwrap().iter() {
                if let Ok(map) = fast_tile_map_query.get(*child) {
                    let mut m = match map.get_mut(&mut *images) {
                        Err(e) => {
                            // Map texture is not available
                            warn!("no map: {:?}", e);
                            continue;
                        }
                        Ok(x) => x,
                    };

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
            vec2(16., 16.),
        )
        .build_and_set(&mut images, |_| rng.gen_range(0..15));

        commands
            .entity(entity)
            .insert((
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            chunk.chunk_pos.x() as f32
                                * chunk.get_chunk_dimensions().x as f32
                                * 16.0,
                            chunk.chunk_pos.y() as f32
                                * chunk.get_chunk_dimensions().y as f32
                                * 16.0,
                            1.0,
                        ),
                        ..default()
                    },
                    ..default()
                },
                ChunkMapSpawned,
            ))
            .with_children(|parent| {
                let mut map_bundle = MapBundle::new(map);
                map_bundle.transform.translation = Vec3::new(
                    chunk.chunk_pos.x() as f32 * chunk.get_chunk_dimensions().x as f32 * 16.0,
                    chunk.chunk_pos.y() as f32 * chunk.get_chunk_dimensions().y as f32 * 16.0,
                    1.0,
                );
                parent
                    .spawn(map_bundle)
                    .insert(Transform::from_translation(Vec3::new(1.0, 1.0, 1.0)))
                    // Have the map manage our mesh so it always has the right size
                    .insert(MeshManagedByMap)
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
    mouse_button: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(
        &GlobalTransform,
        &mut Transform,
        &Camera,
        &mut OrthographicProjection,
    )>,
) {
    for event in mouse_motion_events.iter() {
        if mouse_button.pressed(MouseButton::Left) || mouse_button.pressed(MouseButton::Right) {
            for (_, mut transform, _, _) in camera_query.iter_mut() {
                transform.translation.x -= event.delta.x * transform.scale.x;
                transform.translation.y += event.delta.y * transform.scale.y;
            }
        }
    }

    let mut wheel_y = 0.;
    for event in mouse_wheel_events.iter() {
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
