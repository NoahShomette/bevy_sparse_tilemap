#[cfg(feature = "bevy_fast_tilemap")]
mod grid;
pub mod integrations;
pub mod map;
pub mod tilemap_builder;
pub mod tilemap_manager;
pub mod tiles;

use bevy::app::App;
use bevy::prelude::Plugin;
pub use tiles::TilePos;

use crate::integrations::IntegrationsPlugin;

/// Plugin provided to setup **BevySparseTilemap**
///
/// Mostly only required when enabling optional features and integrations
pub struct SparseTilemapPlugin;

impl Plugin for SparseTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IntegrationsPlugin);
    }
}
