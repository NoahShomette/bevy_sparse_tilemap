use lettuces::cell::Cell;

use crate::map::chunk::{ChunkCell, ChunkPos};

pub trait CellConversionTrait {
    type ConversionSettings;
    fn into_chunk_pos(cell: Cell, conversion_settings: Self::ConversionSettings) -> ChunkPos;

    fn into_chunk_cell(cell: Cell, conversion_settings: Self::ConversionSettings) -> ChunkCell;
}

pub mod square_cell {}

/*

    /// Converts a [`Tilemap`] tiles [`TilePos`] into a [`ChunkPos`]
    pub fn into_chunk_pos(self, max_chunk_size: UVec2) -> ChunkPos {
        ChunkPos::new(self.x / max_chunk_size.x, self.y / max_chunk_size.y)
    }

    /// Converts a [`Tilemap`] tiles [`TilePos`] into a [`ChunkTilePos`]
    pub fn into_chunk_tile_pos(self, max_chunk_size: UVec2) -> ChunkTilePos {
        let chunk_pos_x = self.x / max_chunk_size.x;
        let chunk_pos_y = self.y / max_chunk_size.y;
        ChunkTilePos::new(
            self.x - (chunk_pos_x * max_chunk_size.x),
            self.y - (chunk_pos_y * max_chunk_size.y),
        )
    }

*/
