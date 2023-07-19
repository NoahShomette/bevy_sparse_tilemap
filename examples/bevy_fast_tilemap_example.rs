use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use std::time::Duration;

use bevy_fast_tilemap::map::MapLoading;
use bevy_fast_tilemap::{FastTileMapPlugin, Map, MapBundle, MeshManagedByMap};
use rand::Rng;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    for _ in 0..125000 {
        commands.spawn_empty();
    }

    //commands.spawn(TileMap::<(u8, (u8, u8))>::init_default(500, 500));

    // Create map with (10 * 128) ^ 2 tiles or 1,638,400 tiles.
    let map = Map::builder(
        // Map size (tiles)
        uvec2(500, 500),
        // Tile atlas
        asset_server.load("tiles_16.png"),
        // Tile size (pixels)
        vec2(16., 16.),
    )
    .build(&mut images);

    commands
        .spawn(MapBundle::new(map))
        // Have the map manage our mesh so it always has the right size
        .insert(MeshManagedByMap);
}

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
        .add_startup_system(startup)
        .add_system(mouse_controls_camera)
        .add_system(change_map)
        .run();
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

#[derive(Resource, Default)]
pub struct ChangeTimer(Timer);

/// Update random patches of tile indices in the map
fn change_map(
    mut images: ResMut<Assets<Image>>,
    maps: Query<&Map, Without<MapLoading>>,
    mut change_timer: Local<ChangeTimer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    change_timer.0.tick(time.delta());
    if change_timer.0.finished() {
        for map in maps.iter() {
            // Get the indexer into the map texture
            let mut m = match map.get_mut(&mut *images) {
                Err(e) => {
                    // Map texture is not available
                    warn!("no map: {:?}", e);
                    continue;
                }
                Ok(x) => x,
            };

            let k = rng.gen_range(5..50);
            let x_min = rng.gen_range(0..m.size().x - k);
            let y_min = rng.gen_range(0..m.size().y - k);

            for y in y_min..y_min + k {
                for x in x_min..x_min + k {
                    let i = rng.gen_range(1..12);
                    m.set(x, y, i);
                }
            }
        }
        change_timer.0.set_duration(Duration::from_secs_f32(15.0));
        change_timer.0.set_mode(TimerMode::Repeating);
    }
} // fn change_map
