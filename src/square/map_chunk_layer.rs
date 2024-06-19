//! This module is for data structures to store and interact with a Chunks Layers.

use crate::map::chunk::{ChunkCell, LayerType, ChunkLayer};
use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::math::UVec2;
use bevy::prelude::{Component, Entity, Reflect};
use bevy::utils::HashMap;
use lettuces::storage::grid::Grid;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Reflect, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SquareChunkLayerConversionSettings {
    pub max_chunk_size: UVec2,
}

impl Default for SquareChunkLayerConversionSettings {
    fn default() -> Self {
        Self {
            max_chunk_size: UVec2 { x: 10, y: 10 },
        }
    }
}

/// A struct that holds the chunk map data for the given layer
#[derive(Clone, Component, Default, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Hash, MapEntities)]
pub struct SquareChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    layer_type_data: SquareChunkLayerData<T>,
    tile_entities: HashMap<u64, Entity>,
}

impl<T> MapEntities for SquareChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for tile_entity in self.tile_entities.iter_mut() {
            *tile_entity.1 = entity_mapper.map_entity(*tile_entity.1);
        }
    }
}

impl<T> Hash for SquareChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn hash<H: Hasher>(&self, h: &mut H) {
        let mut pairs: Vec<_> = self.tile_entities.iter().collect();
        pairs.sort_by_key(|i| i.0);
        Hash::hash(&pairs, h);
        Hash::hash(&self.layer_type_data, h);
    }
}
impl<T> ChunkLayer<T> for SquareChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    type ConversionSettings = SquareChunkLayerConversionSettings;

    fn into_chunk_cell(
        cell: lettuces::cell::Cell,
        conversion_settings: &Self::ConversionSettings,
    ) -> ChunkCell {
        let chunk_pos_x = cell.x / conversion_settings.max_chunk_size.x as i32;
        let chunk_pos_y = cell.y / conversion_settings.max_chunk_size.y as i32;
        ChunkCell::new(
            cell.x - (chunk_pos_x * conversion_settings.max_chunk_size.x as i32),
            cell.y - (chunk_pos_y * conversion_settings.max_chunk_size.y as i32),
        )
    }

    fn new(layer_type: LayerType<T>, chunk_dimensions: UVec2) -> Self {
        match layer_type {
            LayerType::Dense(dense_data) => Self {
                layer_type_data: SquareChunkLayerData::new_dense_from_vecs(&dense_data),
                tile_entities: Default::default(),
            },
            LayerType::Sparse(hashmap) => {
                let sparse_data = hashmap
                    .iter()
                    .map(|(chunk_tile_pos, tile_data)| {
                        let number =
                            ((chunk_tile_pos.x() as u64) << 32) | chunk_tile_pos.y() as u64;
                        (number, tile_data.clone())
                    })
                    .collect();
                SquareChunkLayer {
                    layer_type_data: SquareChunkLayerData::Sparse(sparse_data, chunk_dimensions),
                    tile_entities: Default::default(),
                }
            }
        }
    }

    fn get_chunk_dimensions(&self) -> UVec2 {
        self.layer_type_data.get_dimensions()
    }

    fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkCell) -> Option<&mut T> {
        self.layer_type_data.get_tile_data_mut(chunk_tile_pos)
    }

    fn get_tile_data(&self, chunk_tile_pos: ChunkCell) -> Option<&T> {
        self.layer_type_data.get_tile_data(chunk_tile_pos)
    }

    fn set_tile_data(&mut self, chunk_tile_pos: ChunkCell, tile_data: T) {
        self.layer_type_data
            .set_tile_data(chunk_tile_pos, tile_data);
    }

    fn get_tile_entity(&self, chunk_tile_pos: ChunkCell) -> Option<Entity> {
        let number = ((chunk_tile_pos.x() as u64) << 32) | chunk_tile_pos.y() as u64;
        self.tile_entities.get(&number).cloned()
    }

    fn set_tile_entity(&mut self, chunk_tile_pos: ChunkCell, entity: Entity) {
        let number = ((chunk_tile_pos.x() as u64) << 32) | chunk_tile_pos.y() as u64;
        self.tile_entities.insert(number, entity);
    }
}

/// The type of layer data arrangement
///
/// # Sparse
///
/// **A layer where every tile is not filled**
///
/// 0. A hashmap of TilePos -> TileData
/// 1. A UVec2 representing the actual size of the chunk
///
/// # Dense
///
/// **A layer where every tile has TileData**
#[derive(Clone, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Hash)]
pub enum SquareChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    Sparse(HashMap<u64, T>, UVec2),
    Dense(Grid<T>),
}

impl<T> Hash for SquareChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn hash<H: Hasher>(&self, h: &mut H) {
        match self {
            SquareChunkLayerData::Sparse(hashmap, chunk_size) => {
                let mut pairs: Vec<_> = hashmap.iter().collect();
                pairs.sort_by_key(|i| i.0);
                Hash::hash(&pairs, h);
                Hash::hash(&chunk_size, h);
            }
            SquareChunkLayerData::Dense(grid) => {
                Hash::hash(grid, h);
            }
        }
    }
}

impl<T> Default for SquareChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn default() -> Self {
        Self::Dense(Grid::<T>::new(0, 0))
    }
}

impl<T> SquareChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    /// Creates a new [`SquareChunkLayerData::Dense`] with all the tiles having the same data as the default
    /// for T
    pub fn new_dense_default(chunk_size_x: usize, chunk_size_y: usize) -> Self {
        let grid: Grid<T> = Grid::new(chunk_size_x, chunk_size_y);
        Self::Dense(grid)
    }

    /// Creates a new [`SquareChunkLayerData::Dense`] with all the tiles having the same data as the given
    /// tile_data
    pub fn new_dense_uniform(chunk_size_x: usize, chunk_size_y: usize, tile_data: T) -> Self {
        let grid: Grid<T> = Grid::init(chunk_size_x, chunk_size_y, tile_data);
        Self::Dense(grid)
    }

    /// Creates a new [`SquareChunkLayerData::Dense`]from the given vectors of vectors of T
    pub fn new_dense_from_vecs(tile_data: &Vec<Vec<T>>) -> Self {
        let mut given_tile_count = 0u64;

        for tile_data in tile_data.iter() {
            given_tile_count += tile_data.len() as u64;
        }

        assert_eq!(
            (tile_data[0].len() * tile_data.len()) as u64,
            given_tile_count
        );

        let mut grid: Grid<T> = Grid::init(tile_data.len(), tile_data[0].len(), T::default());
        let mut current_x = 0usize;
        let mut current_y = 0usize;
        let row_length = tile_data[0].len();
        grid.fill_with(|| {
            let tile = tile_data[current_y][current_x];
            current_x += 1;
            if current_x == row_length {
                current_x = 0;
                current_y += 1;
            }
            tile
        });

        Self::Dense(grid)
    }
}

impl<T> SquareChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    pub fn get_dimensions(&self) -> UVec2 {
        match self {
            SquareChunkLayerData::Sparse(_, dimensions) => *dimensions,
            SquareChunkLayerData::Dense(grid) => {
                UVec2::new(grid.size().1 as u32, grid.size().0 as u32)
            }
        }
    }

    pub fn set_tile_data(&mut self, chunk_tile_pos: ChunkCell, tile_data: T) {
        match self {
            SquareChunkLayerData::Sparse(layer_data, ..) => {
                let number = ((chunk_tile_pos.x() as u64) << 32) | chunk_tile_pos.y() as u64;
                layer_data.insert(number, tile_data);
            }
            SquareChunkLayerData::Dense(layer_data) => {
                if let Some(tile) =
                    layer_data.get_mut(chunk_tile_pos.y() as usize, chunk_tile_pos.x() as usize)
                {
                    *tile = tile_data
                };
            }
        };
    }

    pub fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkCell) -> Option<&mut T> {
        return match self {
            SquareChunkLayerData::Sparse(layer_data, ..) => {
                let number = ((chunk_tile_pos.x() as u64) << 32) | chunk_tile_pos.y() as u64;
                layer_data.get_mut(&number)
            }
            SquareChunkLayerData::Dense(layer_data) => {
                layer_data.get_mut(chunk_tile_pos.y() as usize, chunk_tile_pos.x() as usize)
            }
        };
    }

    pub fn get_tile_data(&self, chunk_tile_pos: ChunkCell) -> Option<&T> {
        return match self {
            SquareChunkLayerData::Sparse(layer_data, ..) => {
                let number = ((chunk_tile_pos.x() as u64) << 32) | chunk_tile_pos.y() as u64;
                layer_data.get(&number)
            }
            SquareChunkLayerData::Dense(layer_data) => {
                layer_data.get(chunk_tile_pos.y() as usize, chunk_tile_pos.x() as usize)
            }
        };
    }
}
