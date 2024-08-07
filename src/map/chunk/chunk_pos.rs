use bevy::{math::IVec2, prelude::Component};
use lettuces::cell::Cell;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};

/// The position of a [`Chunk`](crate::map::chunk::Chunk) in the [`Tilemap`](crate::map::Tilemap)
///
/// A [`Cell`] can be converted into a [`ChunkPos`] using [`MapData::into_chunk_pos`](crate::map::MapData::into_chunk_pos())
/// The position of a tile in a [`Tilemap`](crate::map::Tilemap)
#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug, Component)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component, Hash))]
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
