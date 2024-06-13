//! This module is specifically for making Tilemaps and helps to give ways to make Tilemap global layers
//! and then convert those into chunks

use bevy::math::UVec2;
use bevy::prelude::{Bundle, Commands, Entity};
use bevy::utils::hashbrown::HashMap;
use lettuces::cell::Cell;

/// An enum that holds all the data for a tilemap layer. This layer is only used in the [`TilemapBuilder`]
///
/// Spawned tilemaps data is separated into [`Chunk`]s as [`ChunkLayerData`](crate::map::chunk::ChunkLayerData)
///
/// # Tilemaps can have two types of layers
///
/// ## Sparse
///
/// *A layer where not every tile exists*
///
/// Consists of three parts:
///
/// 0. A hashmap of TilePos -> TileData
/// 1. A UVec2 representing the size of the Tilemap
/// 2. A hashmap of TilePos -> Entity
///     - The optional entities that hold the extra information when a tile needs it
///
/// ## Dense-
///
/// *A layer where every tile has TileData*
#[derive(Clone, Debug)]
pub enum TilemapLayer<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    Sparse(HashMap<Cell, T>, UVec2, HashMap<Cell, Entity>),
    Dense(Vec<Vec<T>>, HashMap<Cell, Entity>),
}

impl<T> Default for TilemapLayer<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    fn default() -> Self {
        Self::Dense(vec![vec![]], HashMap::default())
    }
}

impl<T> TilemapLayer<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    /// Returns the dimensions of the layer
    pub fn dimensions(&self) -> UVec2 {
        match self {
            TilemapLayer::Sparse(_, dimensions, ..) => *dimensions,
            TilemapLayer::Dense(data, ..) => UVec2::new(data[0].len() as u32, data.len() as u32),
        }
    }

    /// Creates a new empty [`TilemapLayer::Sparse`]
    pub fn new_sparse_empty(tile_map_size_x: usize, tile_map_size_y: usize) -> Self {
        Self::Sparse(
            HashMap::new(),
            UVec2::new(tile_map_size_x as u32, tile_map_size_y as u32),
            HashMap::default(),
        )
    }

    /// Creates a new [`TilemapLayer::Sparse`] from the provided HashMap
    pub fn new_sparse_from_hashmap(
        tile_map_size_x: usize,
        tile_map_size_y: usize,
        hashmap: HashMap<Cell, T>,
    ) -> Self {
        Self::Sparse(
            hashmap,
            UVec2::new(tile_map_size_x as u32, tile_map_size_y as u32),
            HashMap::default(),
        )
    }

    /// Creates a new [`TilemapLayer::Dense`] with all the tiles having the same data as the default
    /// for T
    pub fn new_dense_default(tile_map_size_x: usize, tile_map_size_y: usize) -> Self {
        let mut y_vec: Vec<Vec<T>> = Vec::with_capacity(tile_map_size_y);
        for _ in 0..tile_map_size_y {
            let mut x_vec = Vec::with_capacity(tile_map_size_x);
            x_vec.fill(T::default());
            y_vec.push(x_vec);
        }
        Self::Dense(y_vec, HashMap::default())
    }

    /// Creates a new [`TilemapLayer::Dense`] with all the tiles having the same data as the given
    /// tile_data
    pub fn new_dense_uniform(tile_map_size_x: usize, tile_map_size_y: usize, tile_data: T) -> Self {
        let mut y_vec: Vec<Vec<T>> = Vec::with_capacity(tile_map_size_y);
        for _ in 0..tile_map_size_y {
            let mut x_vec = Vec::with_capacity(tile_map_size_x);
            x_vec.fill(tile_data);
            y_vec.push(x_vec);
        }
        Self::Dense(y_vec, HashMap::default())
    }

    /// Creates a new [`TilemapLayer::Dense`] from the given vectors of vectors of T
    pub fn new_dense_from_vecs(tile_data: Vec<Vec<T>>) -> Self {
        let mut given_tile_count = 0u64;

        for tile_data in tile_data.iter() {
            given_tile_count += tile_data.len() as u64;
        }

        assert_eq!(
            (tile_data[0].len() * tile_data.len()) as u64,
            given_tile_count
        );

        let mut y_vec: Vec<Vec<T>> = Vec::with_capacity(tile_data.len());
        for y in 0..tile_data.len() {
            let mut x_vec = Vec::with_capacity(tile_data[0].len());
            for x in 0..tile_data[0].len() {
                x_vec.push(tile_data[y][x]);
            }
            y_vec.push(x_vec);
        }
        Self::Dense(y_vec, HashMap::default())
    }

    /// Spawns an entity at the given [`TilePos`] with the given [`Bundle`]
    pub fn spawn_entity_at_tile_pos<B: Bundle>(
        &mut self,
        cell: Cell,
        bundle: B,
        commands: &mut Commands,
    ) {
        let entity = commands.spawn(bundle).id();

        match self {
            TilemapLayer::Sparse(_, _, entities) => {
                entities.insert(cell, entity);
            }
            TilemapLayer::Dense(_, entities) => {
                entities.insert(cell, entity);
            }
        }
    }
}
