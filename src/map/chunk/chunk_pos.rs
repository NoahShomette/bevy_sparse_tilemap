use bevy::{
    math::IVec2,
    prelude::{Component, Reflect, ReflectComponent},
};
use lettuces::cell::Cell;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The position of a [`Chunk`] in the [`Tilemap`]
///
/// A [`TilePos`] can be converted into a [`ChunkPos`] using [`TilePos::into_chunk_pos`]
/// The position of a tile in a [`Tilemap`]
#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Component, Hash)]
pub struct ChunkPos(Cell);

impl ChunkPos {
    /// Constructs a new TilePos from the given x and y
    pub fn new(x: i32, y: i32) -> ChunkPos {
        Self { 0: Cell { x, y } }
    }
    /// Returns the x position of Self
    pub fn x(&self) -> i32 {
        self.0.x
    }
    /// Returns the y position of Self
    pub fn y(&self) -> i32 {
        self.0.y
    }
}

impl From<IVec2> for ChunkPos {
    fn from(value: IVec2) -> Self {
        Self {
            0: Cell {
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
