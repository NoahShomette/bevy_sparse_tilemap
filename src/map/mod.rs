//! # `Bevy Sparse Tilemap`
//! 
//! This module contains the features that drive the actual map.
//! 
//! ---
//! 
//! - [`chunk`]
//! 
//! Contains the traits and structures that drive the chunk storage of each map.
//! 
//! - [`tilemap`]
//! 
//! Contains the actual [`Tilemap`] struct which is the highest level storage of a map. See [`Tilemap`] for more details.
//! 
//! ---
//! 
//! ## Broad Overview
//! 
//! There are two main traits that drive bevy sparse tilemap.
//! 
//! - [`MapData`]
//! - [`ChunkLayer`]
//! 
//! MapData is the high level implementation that drives map construction and cell -> chunk pos conversion.
//! 
//! ChunkLayer is the meat and potatoes of BST and controls all of the access of the map.

pub mod chunk;
mod tilemap;

use bevy::{
    math::UVec2,
    prelude::{Component, Entity},
    utils::HashMap,
};
use chunk::{Chunk, ChunkLayer, ChunkPos};
use lettuces::cell::Cell;
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
    for<'a> &'a L: Default,
{
    fn to_bits(&self) -> u32 {
        L::to_bits(self)
    }

    fn all_bits() -> u32 {
        L::all_bits()
    }
}

/// Trait that must be implemented for a map type. It consists of mandatory functions used in building new maps as well as implementing a way to convert a given [`Cell`] into a chunk pos
pub trait MapData: Hash + Component {
    /// Converts a [`Cell`] (A position on the map) into a [`ChunkPos`] (The position of the chunk that that cell is in)
    fn into_chunk_pos(&self, cell: Cell) -> ChunkPos;

    /// The maximum size that a chunk can be
    fn max_chunk_size(&self) -> UVec2;

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
        chunk_settings: MapChunk::ChunkSettings,
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
        chunk_settings: MapChunk::ChunkSettings,
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
            let chunk_pos = self.into_chunk_pos(*cell);
            let chunk = &mut chunks[chunk_pos.y() as usize][chunk_pos.x() as usize];
            chunk.set_tile_entity(
                map_layer,
                MapChunk::into_chunk_cell(*cell, &chunk.chunk_settings),
                *entity,
            );
        }
    }
}
