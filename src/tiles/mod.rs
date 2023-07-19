//! Core Tile concept

use crate::map::{ChunkPos, ChunkTilePos};
use bevy::prelude::{FromReflect, Reflect, UVec2};

/// The position of a tile in a [`Tilemap`]
#[derive(Default, Eq, Hash, PartialEq, Ord, PartialOrd, Copy, Clone, Reflect, FromReflect)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    /// Constructs a new TilePos from the given x and y
    pub fn new(x: u32, y: u32) -> TilePos {
        Self { x, y }
    }

    /// Converts a [`Tilemap`] tiles [`TilePos`] into a [`ChunkPos`]
    pub fn into_chunk_pos(self, max_chunk_size: UVec2) -> ChunkPos {
        ChunkPos {
            x: self.x / max_chunk_size.x,
            y: self.y / max_chunk_size.y,
        }
    }

    /// Converts a [`Tilemap`] tiles [`TilePos`] into a [`ChunkTilePos`]
    pub fn into_chunk_tile_pos(self, max_chunk_size: UVec2) -> ChunkTilePos {
        let chunk_pos_x = self.x / max_chunk_size.x;
        let chunk_pos_y = self.y / max_chunk_size.y;

        ChunkTilePos {
            x: self.x - (chunk_pos_x * max_chunk_size.x),
            y: self.y - (chunk_pos_y * max_chunk_size.y),
        }
    }
}

impl From<UVec2> for TilePos {
    fn from(value: UVec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Into<(usize, usize)> for TilePos {
    fn into(self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
