//! Core Tilemap concept and

pub mod chunk;
mod tilemap;

use bevy::{ecs::component::Component, math::UVec2, prelude::Entity, utils::HashMap};
use chunk::{Chunk, ChunkLayer, ChunkPos};
use lettuces::cell::Cell;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
pub use tilemap::Tilemap;

/// A layer used for identifying and accessing multiple layers of a [`Tilemap`]
///
/// This trait can be derived for enums with `#[derive(MapLayer)]`.
pub trait MapLayer: Default {
    /// Converts the layer to a bitmask.
    fn to_bits(&self) -> u32;
    /// Creates a layer bitmask with all bits set to 1.
    fn all_bits() -> u32;
}

impl<L: MapLayer> MapLayer for &L
where
    for<'a> &'a L: std::default::Default,
{
    fn to_bits(&self) -> u32 {
        L::to_bits(self)
    }

    fn all_bits() -> u32 {
        L::all_bits()
    }
}

/// Trait that must be implemented for a map type. It consists of mandatory functions used in building new maps as well as implementing a way to convert a given [`Cell`] into a chunk pos
pub trait MapData: Hash {
    /// Information needed to convert a [`Cell`] into a chunk position.
    type ChunkPosConversionInfo: Send + Sync + Default + Clone + Hash;
    fn into_chunk_pos(cell: Cell, conversion_settings: &Self::ChunkPosConversionInfo) -> ChunkPos;

    /// Returns the [`Self::ChunkPosConversionInfo`]
    fn conversion_settings(&self) -> &Self::ChunkPosConversionInfo;

    /// Function that breaks a [`Vec<Vec<TileData>>`] down into a [`Vec<Vec<TileData>>`] of the given [`ChunkPos`] chunks data
    fn break_data_vecs_down_into_chunk_data<TileData>(
        &self,
        data: &Vec<Vec<TileData>>,
        chunk_pos: ChunkPos,
        max_chunk_size: UVec2,
    ) -> Vec<Vec<TileData>>
    where
        TileData: Clone + Copy + Sized + Default + Send + Sync + 'static;

    /// Function that breaks a [`Vec<Vec<TileData>>`] into [`Vec<Vec<Chunk<TileData>>>`]
    ///
    /// This function should:
    /// - Create new chunks
    /// - Insert the correct data for each chunk
    /// - Return a [`Vec<Vec<Chunk<TileData>>>`] where each chunk is correctly positioned.
    ///     - Correctly positioned meaning chunk 0:0 contains the tiles for cell positions 0:0 -> 0:max chunk size and max chunk size:0 and so forth for each chunk in order
    fn break_data_vecs_into_chunks<TileData, MapChunk>(
        &self,
        data: &Vec<Vec<TileData>>,
        max_chunk_size: UVec2,
        chunk_conversion_settings: MapChunk::ConversionSettings,
        map_settings: MapChunk::MapSettings,
    ) -> Vec<Vec<Chunk<MapChunk, TileData>>>
    where
        TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
        MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default;

    /// Function that breaks a [`HashMap<TilePos, TileData>`] into [`Vec<Vec<Chunk<TileData>>>`]
    fn break_hashmap_into_chunks<TileData, MapChunk>(
        &self,
        map_layer: impl MapLayer,
        data: &HashMap<Cell, TileData>,
        map_size: UVec2,
        max_chunk_size: UVec2,
        chunk_conversion_settings: MapChunk::ConversionSettings,
        map_settings: MapChunk::MapSettings,
    ) -> Vec<Vec<Chunk<MapChunk, TileData>>>
    where
        TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
        MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default;

    /// Adds the given hashmap of entities to the map
    fn add_entities_to_layer<TileData, MapChunk>(
        &self,
        map_layer: u32,
        chunks: &mut Vec<Vec<Chunk<MapChunk, TileData>>>,
        entities: &HashMap<Cell, Entity>,
    ) where
        TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
        MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default,
    {
        for (cell, entity) in entities.iter() {
            let chunk_pos = Self::into_chunk_pos(*cell, self.conversion_settings());
            let chunk = &mut chunks[chunk_pos.y() as usize][chunk_pos.x() as usize];
            chunk.set_tile_entity(
                map_layer,
                MapChunk::into_chunk_cell(*cell, &chunk.cell_conversion_settings),
                *entity,
            );
        }
    }
}
