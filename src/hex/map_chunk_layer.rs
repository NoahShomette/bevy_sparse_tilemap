//! This module is for data structures to store and interact with a Chunks Layers.

use crate::map::chunk::{ChunkCell, ChunkLayer, LayerType};
use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::math::UVec2;
use bevy::prelude::{Component, Entity, Reflect};
use bevy::utils::HashMap;
use lettuces::cell::Cell;
use lettuces::storage::hex::HexRectangleStorage;
use lettuces::HexOrientation;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};

#[derive(Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Hash, Component))]
/// Settings for a hexagonal map
pub struct HexagonMapSettings {
    pub orientation: HexOrientation,
}

#[derive(Reflect, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HexChunkLayerConversionSettings {
    pub max_chunk_size: UVec2,
}

impl Default for HexChunkLayerConversionSettings {
    fn default() -> Self {
        Self {
            max_chunk_size: UVec2 { x: 10, y: 10 },
        }
    }
}

/// A struct that holds the chunk map data for the given layer
#[derive(Component, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Hash, Component, MapEntities))]
pub struct HexChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    layer_type_data: HexChunkLayerData<T>,
    tile_entities: HashMap<u64, Entity>,
}

impl<T> MapEntities for HexChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for tile_entity in self.tile_entities.iter_mut() {
            *tile_entity.1 = entity_mapper.map_entity(*tile_entity.1);
        }
    }
}

impl<T> Hash for HexChunkLayer<T>
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
impl<T> ChunkLayer<T> for HexChunkLayer<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    type ConversionSettings = HexChunkLayerConversionSettings;

    type MapSettings = HexagonMapSettings;

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

    fn new(
        layer_type: LayerType<T>,
        chunk_dimensions: UVec2,
        settings: &HexagonMapSettings,
    ) -> Self {
        match layer_type {
            LayerType::Dense(dense_data) => Self {
                layer_type_data: HexChunkLayerData::new_dense_from_vecs(
                    &dense_data,
                    settings.orientation.clone(),
                ),
                tile_entities: Default::default(),
            },
            LayerType::Sparse(hashmap) => {
                let sparse_data = hashmap
                    .iter()
                    .map(|(chunk_tile_pos, tile_data)| {
                        ((chunk_tile_pos.x(), chunk_tile_pos.y()), tile_data.clone())
                    })
                    .collect();
                HexChunkLayer {
                    layer_type_data: HexChunkLayerData::Sparse(sparse_data, chunk_dimensions),
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
pub enum HexChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    Sparse(HashMap<(i32, i32), T>, UVec2),
    Dense(HexRectangleStorage<T>),
}

impl<T> Hash for HexChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn hash<H: Hasher>(&self, h: &mut H) {
        match self {
            HexChunkLayerData::Sparse(hashmap, chunk_size) => {
                let mut pairs: Vec<_> = hashmap.iter().collect();
                pairs.sort_by_key(|i| i.0);
                Hash::hash(&pairs, h);
                Hash::hash(&chunk_size, h);
            }
            HexChunkLayerData::Dense(grid) => {
                Hash::hash(grid, h);
            }
        }
    }
}

impl<T> Default for HexChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    fn default() -> Self {
        Self::Dense(HexRectangleStorage::<T>::new(0, 0, HexOrientation::Pointy))
    }
}

impl<T> HexChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    /// Creates a new [`SquareChunkLayerData::Dense`] with all the tiles having the same data as the default
    /// for T
    pub fn new_dense_default(
        chunk_size_x: usize,
        chunk_size_y: usize,
        orientation: HexOrientation,
    ) -> Self {
        let grid = HexRectangleStorage::new(chunk_size_x, chunk_size_y, orientation);
        Self::Dense(grid)
    }

    /// Creates a new [`SquareChunkLayerData::Dense`] with all the tiles having the same data as the given
    /// tile_data
    pub fn new_dense_uniform(
        chunk_size_x: usize,
        chunk_size_y: usize,
        tile_data: T,
        orientation: HexOrientation,
    ) -> Self {
        let grid =
            HexRectangleStorage::new_uniform(chunk_size_x, chunk_size_y, tile_data, orientation);
        Self::Dense(grid)
    }

    /// Creates a new [`SquareChunkLayerData::Dense`]from the given vectors of vectors of T
    pub fn new_dense_from_vecs(tile_data: &Vec<Vec<T>>, orientation: HexOrientation) -> Self {
        let mut given_tile_count = 0u64;

        for tile_data in tile_data.iter() {
            given_tile_count += tile_data.len() as u64;
        }

        assert_eq!(
            (tile_data[0].len() * tile_data.len()) as u64,
            given_tile_count
        );

        let mut grid = HexRectangleStorage::new_uniform(
            tile_data.len(),
            tile_data[0].len(),
            T::default(),
            orientation,
        );
        let mut current_x = 0usize;
        let mut current_y = 0usize;
        let row_length = tile_data[0].len();
        grid.grid.fill_with(|| {
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

impl<T> HexChunkLayerData<T>
where
    T: Hash + Clone + Copy + Sized + Default + Send + Sync,
{
    pub fn get_dimensions(&self) -> UVec2 {
        match self {
            HexChunkLayerData::Sparse(_, dimensions) => *dimensions,
            HexChunkLayerData::Dense(grid) => {
                UVec2::new(grid.dimensions().y as u32, grid.dimensions().x as u32)
            }
        }
    }

    pub fn set_tile_data(&mut self, chunk_tile_pos: ChunkCell, tile_data: T) {
        match self {
            HexChunkLayerData::Sparse(layer_data, ..) => {
                layer_data.insert((chunk_tile_pos.x(), chunk_tile_pos.y()), tile_data);
            }
            HexChunkLayerData::Dense(layer_data) => {
                if let Some(tile) =
                    layer_data.get_mut(Cell::new(chunk_tile_pos.x(), chunk_tile_pos.y()))
                {
                    *tile = tile_data
                };
            }
        };
    }

    pub fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkCell) -> Option<&mut T> {
        return match self {
            HexChunkLayerData::Sparse(layer_data, ..) => {
                layer_data.get_mut(&(chunk_tile_pos.x(), chunk_tile_pos.y()))
            }
            HexChunkLayerData::Dense(layer_data) => {
                layer_data.get_mut(Cell::new(chunk_tile_pos.x(), chunk_tile_pos.y()))
            }
        };
    }

    pub fn get_tile_data(&self, chunk_tile_pos: ChunkCell) -> Option<&T> {
        return match self {
            HexChunkLayerData::Sparse(layer_data, ..) => {
                layer_data.get(&(chunk_tile_pos.x(), chunk_tile_pos.y()))
            }
            HexChunkLayerData::Dense(layer_data) => {
                layer_data.get(Cell::new(chunk_tile_pos.x(), chunk_tile_pos.y()))
            }
        };
    }
}
