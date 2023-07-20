mod map;
mod tiles;
#[cfg(feature = "bevy_fast_tilemap")]
mod bevy_fast_tilemap;
mod grid;

use bevy::app::App;
use bevy::prelude::Plugin;
pub use map::{Chunk, ChunkPos, ChunkTilePos, Chunks, Tilemap};
pub use tiles::TilePos;

#[cfg(feature = "bevy_fast_tilemap")]
pub use crate::bevy_fast_tilemap::BevyFastTilemapFeaturePlugin;

pub struct SparseTilemapPlugin;

impl Plugin for SparseTilemapPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_fast_tilemap")]
        app.add_plugin(BevyFastTilemapFeaturePlugin);
    }
}
