//! This module is for data structures to store and interact with a Chunks Layers.

use bevy::math::UVec2;
use bevy::prelude::Entity;
use bevy::utils::HashMap;
use grid::Grid;
use crate::map::chunk::chunk_tile_pos::ChunkTilePos;

/// A struct that holds the chunk map data for the given layer
#[derive(Clone, Default)]
pub struct ChunkLayerData<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    layer_type_data: ChunklayerTypes<T>,
    tile_entities: HashMap<ChunkTilePos, Entity>,
}

// Implementations to make new LayerChunkData
impl<T> ChunkLayerData<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    pub fn new_sparse_layer_empty(chunk_dimensions: UVec2) -> ChunkLayerData<T> {
        ChunkLayerData {
            layer_type_data: ChunklayerTypes::Sparse(HashMap::new(), chunk_dimensions),
            tile_entities: Default::default(),
        }
    }

    pub fn new_sparse_layer_from_data(
        sparse_data: HashMap<ChunkTilePos, T>,
        chunk_dimensions: UVec2,
    ) -> ChunkLayerData<T> {
        ChunkLayerData {
            layer_type_data: ChunklayerTypes::Sparse(sparse_data, chunk_dimensions),
            tile_entities: Default::default(),
        }
    }

    pub fn new_dense_default_layer(chunk_size_x: usize, chunk_size_y: usize) -> Self {
        Self {
            layer_type_data: ChunklayerTypes::new_dense_default(chunk_size_x, chunk_size_y),
            tile_entities: Default::default(),
        }
    }

    pub fn new_dense_uniform_layer(chunk_size_x: usize, chunk_size_y: usize, tile_data: T) -> Self {
        Self {
            layer_type_data: ChunklayerTypes::new_dense_uniform(
                chunk_size_x,
                chunk_size_y,
                tile_data,
            ),
            tile_entities: Default::default(),
        }
    }

    pub fn new_dense_from_vecs_layer(tile_data: &Vec<Vec<T>>) -> Self {
        Self {
            layer_type_data: ChunklayerTypes::new_dense_from_vecs(tile_data),
            tile_entities: Default::default(),
        }
    }
}

// Implementations to interact with the LayerChunkData
impl<T> ChunkLayerData<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    pub fn get_chunk_dimensions(&self) -> UVec2 {
        self.layer_type_data.get_dimensions()
    }

    pub fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkTilePos) -> Option<&mut T> {
        self.layer_type_data.get_tile_data_mut(chunk_tile_pos)
    }

    pub fn get_tile_data(&self, chunk_tile_pos: ChunkTilePos) -> Option<&T> {
        self.layer_type_data.get_tile_data(chunk_tile_pos)
    }

    pub fn set_tile_data(&mut self, chunk_tile_pos: ChunkTilePos, tile_data: T) {
        self.layer_type_data
            .set_tile_data(chunk_tile_pos, tile_data);
    }

    pub fn get_tile_entity(&self, chunk_tile_pos: ChunkTilePos) -> Option<Entity> {
        self.tile_entities.get(&chunk_tile_pos).cloned()
    }

    pub fn set_tile_entity(&mut self, chunk_tile_pos: ChunkTilePos, entity: Entity) {
        self.tile_entities.insert(chunk_tile_pos, entity);
    }
}

/// The type of layer data arrangement
///
/// # Sparse
///
/// **A layer where every tile is not filled**
///
/// 0. A hashmap of TilePos -> TileData
/// 1. A UVec2 representing the size of the chunk
///
/// # Dense
///
/// **A layer where every tile has TileData**
#[derive(Clone)]
pub enum ChunklayerTypes<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    /// A sparse layer. 0 is a hash
    Sparse(HashMap<ChunkTilePos, T>, UVec2),
    Dense(Grid<T>),
}

impl<T> Default for ChunklayerTypes<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    fn default() -> Self {
        Self::Dense(Grid::<T>::new(0, 0))
    }
}

impl<T> ChunklayerTypes<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    /// Creates a new [`ChunklayerTypes::Dense`] with all the tiles having the same data as the default
    /// for T
    pub fn new_dense_default(chunk_size_x: usize, chunk_size_y: usize) -> Self {
        let grid: Grid<T> = Grid::new(chunk_size_x, chunk_size_y);
        Self::Dense(grid)
    }

    /// Creates a new [`ChunklayerTypes::Dense`] with all the tiles having the same data as the given
    /// tile_data
    pub fn new_dense_uniform(chunk_size_x: usize, chunk_size_y: usize, tile_data: T) -> Self {
        let grid: Grid<T> = Grid::init(chunk_size_x, chunk_size_y, tile_data);
        Self::Dense(grid)
    }

    /// Creates a new [`ChunklayerTypes::Dense`]from the given vectors of vectors of T
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

impl<T> ChunklayerTypes<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    pub fn get_dimensions(&self) -> UVec2 {
        match self {
            ChunklayerTypes::Sparse(_, dimensions) => *dimensions,
            ChunklayerTypes::Dense(grid) => UVec2::new(grid.size().1 as u32, grid.size().0 as u32),
        }
    }

    pub fn set_tile_data(&mut self, chunk_tile_pos: ChunkTilePos, tile_data: T) {
        match self {
            ChunklayerTypes::Sparse(layer_data, ..) => {
                layer_data.insert(chunk_tile_pos, tile_data);
            }
            ChunklayerTypes::Dense(layer_data) => {
                if let Some(mut tile) =
                    layer_data.get_mut(chunk_tile_pos.y() as usize, chunk_tile_pos.x() as usize)
                {
                    *tile = tile_data
                };
            }
        };
    }

    pub fn get_tile_data_mut(&mut self, chunk_tile_pos: ChunkTilePos) -> Option<&mut T> {
        return match self {
            ChunklayerTypes::Sparse(layer_data, ..) => layer_data.get_mut(&chunk_tile_pos),
            ChunklayerTypes::Dense(layer_data) => {
                layer_data.get_mut(chunk_tile_pos.y() as usize, chunk_tile_pos.x() as usize)
            }
        };
    }

    pub fn get_tile_data(&self, chunk_tile_pos: ChunkTilePos) -> Option<&T> {
        return match self {
            ChunklayerTypes::Sparse(layer_data, ..) => layer_data.get(&chunk_tile_pos),
            ChunklayerTypes::Dense(layer_data) => {
                layer_data.get(chunk_tile_pos.y() as usize, chunk_tile_pos.x() as usize)
            }
        };
    }
}
