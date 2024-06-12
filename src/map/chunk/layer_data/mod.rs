use std::hash::Hash;

use bevy::{ecs::entity::MapEntities, math::UVec2, prelude::Entity, utils::HashMap};

use super::ChunkCell;

pub mod hex;
pub mod square;

pub enum LayerType<T> {
    Dense(Vec<Vec<T>>),
    Sparse(HashMap<ChunkCell, T>),
}

/// Trait that controls access to a specific layer of a tilemap.
pub trait ChunkLayerData<T>: Hash + MapEntities {
    fn new(layer_type: LayerType<T>, chunk_dimensions: UVec2) -> Self;

    fn get_chunk_dimensions(&self) -> UVec2;

    fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkCell) -> Option<&mut T>;

    fn get_tile_data(&self, chunk_tile_pos: ChunkCell) -> Option<&T>;

    fn set_tile_data(&mut self, chunk_tile_pos: ChunkCell, tile_data: T);

    fn get_tile_entity(&self, chunk_tile_pos: ChunkCell) -> Option<Entity>;

    fn set_tile_entity(&mut self, chunk_tile_pos: ChunkCell, entity: Entity);
}
