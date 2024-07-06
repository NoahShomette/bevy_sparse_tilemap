//! Example

use map_chunk_layer::SquareChunkLayer;
use map_data::SquareMapData;

use crate::{map::chunk::Chunk, tilemap_builder::TilemapBuilder, tilemap_manager::TilemapManager};

pub mod map_chunk_layer;
pub mod map_data;

/// Type alias for [`TilemapManager`] for the built in square map types.
pub type SquareTilemapManager<'w, 's, TileData, MapLayers> =
    TilemapManager<'w, 's, TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData>;

/// Type alias for [`Chunk`] using the built in [`SquareChunkLayer`]
pub type SquareChunk<TileData> = Chunk<SquareChunkLayer<TileData>, TileData>;

/// Type alias for [`TilemapBuilder`] for the built in square map types
pub type SquareTilemapBuilder<TileData, MapLayers> =
    TilemapBuilder<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData>;
