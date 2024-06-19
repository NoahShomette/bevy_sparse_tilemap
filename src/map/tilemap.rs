//! Core Tilemap
//!
//! Tilemaps are a sparse-set representation of a tilemap. The minimum amount of data needed
//! saved into the Tilemaps chunks to be able to build out advanced functionality as needed.
//!
//!

use crate::map::chunk::ChunkPos;
use crate::map::chunk::Chunks;
use bevy::ecs::entity::{EntityMapper, MapEntities};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};

use bevy::math::UVec2;
use bevy::prelude::{Component, Entity};

use lettuces::cell::Cell;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::MapData;

/// The data structure containing information for each chunk.
///
/// Each tile should only contain the bare minimum data needed for you to figure out what it is. Any
/// data that is not the same for every single tile of that type should be stored as a component
/// on that tiles entity which is managed through the [`Chunk`](super::chunk::Chunk)
#[derive(Component, Default, Hash, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component, Hash, MapEntities))]
pub struct Tilemap<Map>
where
    Map: MapData,
{
    /// Struct containing [`Entity`] mappings to the [`Chunk`](super::chunk::Chunk)s that hold tile data
    chunks: Chunks,
    chunk_pos_conversion_settings: Map::ConversionSettings,
}

impl<Map> MapEntities for Tilemap<Map>
where
    Map: MapData,
{
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.chunks.map_entities(entity_mapper);
    }
}

impl<Map> Tilemap<Map>
where
    Map: MapData,
{
    /// Creates a new [`Tilemap`] out of the given chunks struct
    pub fn new(chunks: Chunks, conversion_settings: Map::ConversionSettings) -> Tilemap<Map> {
        Self {
            chunks,
            chunk_pos_conversion_settings: conversion_settings,
        }
    }

    /// Gets the chunk entity that contains this cell
    pub fn get_chunk_for_cell(&self, cell: Cell) -> Option<Entity> {
        self.get_chunk(Map::into_chunk_pos(
            cell,
            &self.chunk_pos_conversion_settings,
        ))
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
