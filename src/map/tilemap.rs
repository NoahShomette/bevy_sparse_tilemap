//! Core Tilemap
//!
//! Tilemaps are a sparse-set representation of a tilemap. The minimum amount of data needed
//! saved into the Tilemaps chunks to be able to build out advanced functionality as needed.
//!
//!

use crate::map::chunk::{Chunks};
use crate::{TilePos};
use bevy::math::UVec2;
use bevy::prelude::{Component, Entity};
use std::marker::PhantomData;
use crate::map::chunk::ChunkPos;

/// The data structure containing the entirety of the tilemaps data for each chunk and tile as well 
/// as manages chunk access and setup
///
/// Each tile should only contain the bare minimum data needed for you to figure out what it is. Any
/// data that is not the same for every single tile of that type should be stored as a component
/// on that tiles entity which is managed through the [`Chunk`](super::chunk::Chunk)
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct Tilemap<TilemapMarker>
where
    TilemapMarker: Send + Sync + 'static,
{
    /// Struct containing [`Entity`] mappings to the [`Chunk`](super::chunk::Chunk)s that hold tile data
    chunks: Chunks,
    _marker: PhantomData<TilemapMarker>,
}

impl<TilemapMarker> Tilemap<TilemapMarker>
where
    TilemapMarker: Send + Sync + 'static,
{

    /// Creates a new [`Tilemap`] out of the given chunks struct
    pub fn new(chunks: Chunks) -> Tilemap<TilemapMarker> {
        Self {
            chunks,
            _marker: Default::default(),
        }
    }

    /// Gets the chunk entity that has the tile_info for the given TilePos
    pub fn get_chunk_for_tile_pos(&self, tile_pos: TilePos) -> Option<Entity> {
        self.chunks.get_chunk_from_tile_pos(tile_pos)
    }

    /// Gets the chunk entity that has the tile_info for the given TilePos
    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<Entity> {
        self.chunks.get_chunk(chunk_pos)
    }

    /// Returns the max size that a chunk can be
    pub fn get_chunks_max_size(&self) -> UVec2 {
        self.chunks.max_chunk_size()
    }
}
