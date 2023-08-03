#[cfg(feature = "bevy_fast_tilemap")]
mod bevy_fast_tilemap;
mod grid;
pub mod map;
pub mod tiles;
pub mod tilemap_manager;
pub mod tilemap_builder;

use bevy::app::App;
use bevy::prelude::Plugin;
pub use tiles::TilePos;

#[cfg(feature = "bevy_fast_tilemap")]
pub use crate::bevy_fast_tilemap::BevyFastTilemapFeaturePlugin;

/// Plugin provided to setup **BevySparseTilemap**
///
/// Mostly only required when enabling optional features
pub struct SparseTilemapPlugin;

impl Plugin for SparseTilemapPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_fast_tilemap")]
        app.add_plugin(BevyFastTilemapFeaturePlugin);
    }
}
