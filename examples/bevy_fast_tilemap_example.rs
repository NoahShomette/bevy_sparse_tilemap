use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_fast_tilemap::{FastTileMapPlugin, Map, MapBundle, MeshManagedByMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sparse_tilemap::{Chunk, Tilemap};
use rand::Rng;
use std::thread::spawn;

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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(FastTileMapPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(startup)
        .add_system(mouse_controls_camera)
        .add_system(spawn_or_update_fast_tilemaps)
        .run();
}

#[derive(Default, Copy, Clone, Reflect, FromReflect)]
struct TileData(u8, u8);

#[derive(Component, Default, Copy, Clone, Reflect, FromReflect)]
pub struct FastTileMap;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let map_x = 13000;
    let map_y = 13000;

    let entity = Tilemap::spawn_tilemap(
        generate_random_tile_data(UVec2::new(map_x, map_y)),
        UVec2::new(4000, 4000),
        &mut commands,
    );

    commands.entity(entity).insert(SpatialBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_or_update_fast_tilemaps(
    chunk_query: Query<(Entity, &Chunk<TileData>, Option<&Children>), Changed<Chunk<TileData>>>,
    fast_tile_map_query: Query<&mut Map, With<FastTileMap>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    'main_loop: for (entity, chunk, children) in chunk_query.iter() {
        /*
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(map) = fast_tile_map_query.get(*child) {
                    let mut m = match map.get_mut(&mut *images) {
                        Err(e) => {
                            // Map texture is not available
                            warn!("no map: {:?}", e);
                            continue;
                        }
                        Ok(x) => x,
                    };

                    for y in 0..chunk.tiles.size().1 {
                        for x in 0..chunk.tiles.size().0 {
                            let i = rng.gen_range(1..12);
                            m.set(x as u32, y as u32, i);
                        }
                    }
                    continue 'main_loop;
                }
            }
        }
        
         */

        // Create map with the given dimensions of our chunk
        let map = Map::builder(
            // Map size (tiles)
            uvec2(chunk.tiles.size().0 as u32, chunk.tiles.size().1 as u32),
            // Tile atlas
            asset_server.load("tiles_16.png"),
            // Tile size (pixels)
            vec2(16., 16.),
        )
        .build_and_set(&mut images, |pos| rng.gen_range(0..15));
        println!("{:?}", map.map_texture.id());
        commands
            .entity(entity)
            .insert(SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(
                        chunk.chunk_pos.x as f32 * chunk.tiles.size().0 as f32 * 16.0,
                        chunk.chunk_pos.y as f32 * chunk.tiles.size().1 as f32 * 16.0,
                        1.0,
                    ),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(MapBundle::new(map))
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
        for (_, mut transform, _, mut _ortho) in camera_query.iter_mut() {
            let factor = f32::powf(2., -wheel_y / 2.);
            transform.scale *= vec3(factor, factor, 1.0);
            transform.scale = transform
                .scale
                .max(Vec3::splat(1. / 128.))
                .min(Vec3::splat(128.));
        }
    }
}
