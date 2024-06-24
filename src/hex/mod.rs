use map_chunk_layer::HexChunkLayer;
use map_data::HexMapData;

use crate::{
    map::{chunk::Chunk, Tilemap},
    tilemap_builder::TilemapBuilder,
    tilemap_manager::TilemapManager,
};

pub mod map_chunk_layer;
pub mod map_data;

/// Type alias for [`TilemapManager`] for the built in hexagon map types.
pub type HexTilemapManager<'w, 's, TileData, MapLayers> =
    TilemapManager<'w, 's, TileData, MapLayers, HexChunkLayer<TileData>, HexMapData>;

/// Type alias for [`Chunk`] using the built in [`HexChunkLayer`]
pub type HexChunk<TileData> = Chunk<HexChunkLayer<TileData>, TileData>;

/// Type alias for [`TilemapBuilder`] for the built in hexagon map types
pub type HexTilemapBuilder<TileData, MapLayers> =
    TilemapBuilder<TileData, MapLayers, HexChunkLayer<TileData>, HexMapData>;

/// Type alias for [`Tilemap`] for the built in hexagon map tilemap
pub type HexTilemap = Tilemap<HexMapData>;
