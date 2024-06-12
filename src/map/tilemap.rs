//! Core Tilemap
//!
//! Tilemaps are a sparse-set representation of a tilemap. The minimum amount of data needed
//! saved into the Tilemaps chunks to be able to build out advanced functionality as needed.
//!
//!

use crate::map::chunk::ChunkPos;
use crate::map::chunk::Chunks;
use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::math::UVec2;
use bevy::prelude::{Component, Entity, Reflect, ReflectComponent};

use lettuces::cell::Cell;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The data structure containing information for each chunk.
///
/// Each tile should only contain the bare minimum data needed for you to figure out what it is. Any
/// data that is not the same for every single tile of that type should be stored as a component
/// on that tiles entity which is managed through the [`Chunk`](super::chunk::Chunk)
#[derive(Component, Default, Hash, Clone, Debug, Eq, PartialEq, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Component, Hash, MapEntities)]
pub struct Tilemap {
    /// Struct containing [`Entity`] mappings to the [`Chunk`](super::chunk::Chunk)s that hold tile data
    chunks: Chunks,
}

impl MapEntities for Tilemap {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.chunks.map_entities(entity_mapper);
    }
}

impl Tilemap {
    /// Creates a new [`Tilemap`] out of the given chunks struct
    pub fn new(chunks: Chunks) -> Tilemap {
        Self { chunks }
    }

    /// Gets the chunk entity that has the tile_info for the given TilePos
    pub fn get_chunk_for_tile_pos(&self, cell: Cell) -> Option<Entity> {
        self.chunks.get_chunk_from_cell(cell)
    }

    /// Gets the chunk entity that has the tile_info for the given TilePos
    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<Entity> {
        self.chunks.get_chunk(chunk_pos)
    }

    /// Returns the max size that a chunk can be
    pub fn get_chunks_max_size(&self) -> UVec2 {
        self.chunks.max_chunk_size()
    }

    /// Returns an immutable reference to [`Chunks`]
    pub fn chunks(&self) -> &Chunks {
        &self.chunks
    }

    /// Returns a mutable reference to [`Chunks`]
    pub fn chunks_mut(&mut self) -> &mut Chunks {
        &mut self.chunks
    }
}
