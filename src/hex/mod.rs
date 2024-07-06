use lettuces::{HexOrientation, OffsetHexMode, Quat};
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

/// Converts a [`HexOrientation`] into a [`OffsetHexMode`]. This sets it to Odd Rows and Odd Columns respectively which are the only two that this crate supports
pub fn hex_offset_from_orientation(orientation: HexOrientation) -> OffsetHexMode {
    match orientation {
        HexOrientation::Pointy => lettuces::OffsetHexMode::OddRows,
        HexOrientation::Flat => lettuces::OffsetHexMode::OddColumns,
    }
}

/// Returns the correct hexagon rotation for the given orientation
pub fn hex_rotation(orientation: HexOrientation) -> Quat {
    Quat::from_rotation_z(match orientation {
        HexOrientation::Pointy => 0.0,
        HexOrientation::Flat => 0.52359878,
    })
}
