use std::hash::Hash;

use bevy::{
    ecs::entity::MapEntities, math::UVec2, prelude::Entity, reflect::Reflect, utils::HashMap,
};
use lettuces::cell::Cell;

use super::ChunkCell;

pub enum LayerType<T> {
    Dense(Vec<Vec<T>>),
    Sparse(HashMap<ChunkCell, T>),
}

/// Trait that controls access to a specific layer of a tilemap.
pub trait MapChunkLayer<T>: Hash + MapEntities {
    /// Information needed to convert a [`Cell`] into a [`ChunkCell`].
    type ConversionSettings: Send + Sync + Default + Reflect + Clone + Copy;

    /// Converts a [`Cell`] into a [`ChunkCell`]
    fn into_chunk_cell(cell: Cell, conversion_settings: &Self::ConversionSettings) -> ChunkCell;

    fn new(layer_type: LayerType<T>, chunk_dimensions: UVec2) -> Self;

    fn get_chunk_dimensions(&self) -> UVec2;

    fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkCell) -> Option<&mut T>;

    fn get_tile_data(&self, chunk_tile_pos: ChunkCell) -> Option<&T>;

    fn set_tile_data(&mut self, chunk_tile_pos: ChunkCell, tile_data: T);

    fn get_tile_entity(&self, chunk_tile_pos: ChunkCell) -> Option<Entity>;

    fn set_tile_entity(&mut self, chunk_tile_pos: ChunkCell, entity: Entity);
}
