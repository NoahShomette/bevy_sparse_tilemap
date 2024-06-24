use std::hash::Hash;

use bevy::{ecs::entity::MapEntities, math::UVec2, prelude::Entity, utils::HashMap};
use lettuces::cell::Cell;

use super::ChunkCell;

pub enum LayerType<T> {
    Dense(Vec<Vec<T>>),
    Sparse(HashMap<ChunkCell, T>),
}

/// Trait that controls access to a specific layer of a tilemap chunk.
pub trait ChunkLayer<T>: Hash + MapEntities {
    /// Information needed to convert a [`Cell`] into a [`ChunkCell`].
    type ConversionInfo: Send + Sync + Default + Clone + Copy + Hash;

    /// Settings used to construct the map
    type MapSettings: Send + Sync + Default + Clone + Copy + Hash;

    /// Converts a [`Cell`] into a [`ChunkCell`]
    fn into_chunk_cell(cell: Cell, conversion_settings: &Self::ConversionInfo) -> ChunkCell;

    fn new(
        layer_type: LayerType<T>,
        chunk_dimensions: UVec2,
        map_settings: &Self::MapSettings,
    ) -> Self;

    /// Returns the dimensions of this specific chunk
    fn get_chunk_dimensions(&self) -> UVec2;

    fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkCell) -> Option<&mut T>;

    fn get_tile_data(&self, chunk_tile_pos: ChunkCell) -> Option<&T>;

    fn set_tile_data(&mut self, chunk_tile_pos: ChunkCell, tile_data: T);

    fn get_tile_entity(&self, chunk_tile_pos: ChunkCell) -> Option<Entity>;

    fn set_tile_entity(&mut self, chunk_tile_pos: ChunkCell, entity: Entity);
}
