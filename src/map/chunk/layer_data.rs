use std::hash::Hash;

use bevy::{ecs::entity::MapEntities, math::UVec2, prelude::Entity, utils::HashMap};
use lettuces::cell::Cell;

use super::ChunkCell;

/// The data for a specific chunk. Contains only the data for that chunk
pub enum ChunkLayerType<T> {
    /// A layer where ***NOT*** every position on the chunk has data
    Dense(Vec<Vec<T>>),
    /// A layer where ***EVERY***  position on the chunk must have data
    Sparse(HashMap<ChunkCell, T>),
}

/// Trait that controls access to a specific layer of a tilemap chunk.
pub trait ChunkLayer<TileData>: Hash + MapEntities {
    /// Settings needed by the chunk to:
    ///
    /// - Convert a [`Cell`] into a [`ChunkCell`]
    /// - Create new chunks
    type ChunkSettings: Send + Sync + Default + Clone + Copy + Hash;

    /// Converts a [`Cell`] into a [`ChunkCell`]
    fn into_chunk_cell(cell: Cell, chunk_settings: &Self::ChunkSettings) -> ChunkCell;

    /// Creates a new chunk out of the given [`LayerType`]. The [`TileData`] contained in the layer is only the data that this chunk should contain.
    fn new(
        layer_type: ChunkLayerType<TileData>,
        chunk_dimensions: UVec2,
        map_settings: &Self::ChunkSettings,
    ) -> Self;

    /// Returns the dimensions of this specific chunk
    fn get_chunk_dimensions(&self) -> UVec2;

    /// Gets mutable access to the [`TileData`] at the given [`ChunkCell`]
    fn get_tile_data_mut(&mut self, chunk_cell: ChunkCell) -> Option<&mut TileData>;

    /// Gets immutable access to the [`TileData`] at the given [`ChunkCell`]
    fn get_tile_data(&self, chunk_cell: ChunkCell) -> Option<&TileData>;

    /// Sets the [`TileData`] at the given [`ChunkCell`]
    fn set_tile_data(&mut self, chunk_cell: ChunkCell, tile_data: TileData);

    /// Gets the [`Entity`] at the given [`ChunkCell`]
    fn get_tile_entity(&self, chunk_cell: ChunkCell) -> Option<Entity>;

    /// Sets the [`Entity`] at the given [`ChunkCell`]
    fn set_tile_entity(&mut self, chunk_cell: ChunkCell, entity: Entity);
}
