use bevy::{math::IVec2, prelude::Component};
use lettuces::cell::Cell;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};

/// A position inside a [`Chunk`](crate::map::chunk::Chunk)
///
/// You can get a [`ChunkCell`] using the method [`ChunkLayer::into_chunk_cell()`](crate::map::chunk::ChunkLayer::into_chunk_cell())
#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug, Component)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component, Hash))]
pub struct ChunkCell(Cell);

impl ChunkCell {
    /// Constructs a new ChunkTilePos from the given x and y
    pub fn new(x: i32, y: i32) -> ChunkCell {
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

impl From<IVec2> for ChunkCell {
    fn from(value: IVec2) -> Self {
        Self {
            0: Cell {
                x: value.x,
                y: value.y,
            },
        }
    }
}

impl Into<(usize, usize)> for ChunkCell {
    fn into(self) -> (usize, usize) {
        (self.0.x as usize, self.0.y as usize)
    }
}

impl Display for ChunkCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("x:{}, y:{}", self.0.x, self.0.y))
    }
}
