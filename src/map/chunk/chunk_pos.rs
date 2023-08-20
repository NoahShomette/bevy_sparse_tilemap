use crate::TilePos;
use bevy::prelude::{Component, Reflect, ReflectComponent, UVec2};
use std::fmt::{Display, Formatter};

/// The position of a [`Chunk`] in the [`Tilemap`]
///
/// A [`TilePos`] can be converted into a [`ChunkPos`] using [`TilePos::into_chunk_pos`]
/// The position of a tile in a [`Tilemap`]
#[derive(Default, Eq, Hash, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Component, Reflect)]
#[reflect(Component, Hash)]
pub struct ChunkPos(TilePos);

impl ChunkPos {
    /// Constructs a new TilePos from the given x and y
    pub fn new(x: u32, y: u32) -> ChunkPos {
        Self {
            0: TilePos { x, y },
        }
    }
    /// Returns the x position of Self
    pub fn x(&self) -> u32 {
        self.0.x
    }
    /// Returns the y position of Self
    pub fn y(&self) -> u32 {
        self.0.y
    }
}

impl From<UVec2> for ChunkPos {
    fn from(value: UVec2) -> Self {
        Self {
            0: TilePos {
                x: value.x,
                y: value.y,
            },
        }
    }
}

impl Into<(usize, usize)> for ChunkPos {
    fn into(self) -> (usize, usize) {
        (self.0.x as usize, self.0.y as usize)
    }
}

impl Display for ChunkPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("x:{}, y:{}", self.0.x, self.0.y))
    }
}
