//! This module is specifically for making Tilemaps and helps to give ways to make Tilemap global layers
//! and then convert those into chunks

use crate::map::chunk::{Chunk, ChunkPos};
use crate::TilePos;
use bevy::math::{vec2, UVec2};
use bevy::prelude::{Bundle, Commands, Entity};
use bevy::utils::hashbrown::HashMap;
use std::hash::Hash;

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
    Sparse(HashMap<TilePos, T>, UVec2, HashMap<TilePos, Entity>),
    Dense(Vec<Vec<T>>, HashMap<TilePos, Entity>),
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
        hashmap: HashMap<TilePos, T>,
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
        tile_pos: TilePos,
        bundle: B,
        commands: &mut Commands,
    ) {
        let entity = commands.spawn(bundle).id();

        match self {
            TilemapLayer::Sparse(_, _, entities) => {
                entities.insert(tile_pos, entity);
            }
            TilemapLayer::Dense(_, entities) => {
                entities.insert(tile_pos, entity);
            }
        }
    }
}

/// Adds the given layer to the tilemap
pub fn add_layer_to_chunks<TileData>(
    map_layer: u32,
    chunks: &mut Vec<Vec<Chunk<TileData>>>,
    tilemap_layer: &TilemapLayer<TileData>,
    max_chunk_size: UVec2,
) where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
{
    match tilemap_layer {
        TilemapLayer::Sparse(data, .., entities) => {
            for y in chunks.iter_mut() {
                for chunk in y.iter_mut() {
                    chunk.add_sparse_layer(map_layer, None);
                }
            }
            for (tile_pos, tile_data) in data.iter() {
                let chunk_pos = tile_pos.into_chunk_pos(max_chunk_size);
                chunks[chunk_pos.y() as usize][chunk_pos.x() as usize]
                    .data
                    .get_mut(&map_layer)
                    .unwrap()
                    .set_tile_data(
                        tile_pos.into_chunk_tile_pos(max_chunk_size),
                        tile_data.clone(),
                    );
            }
            add_entities_to_layer(map_layer, chunks, entities, max_chunk_size);
        }
        TilemapLayer::Dense(data, entities) => {
            for y in chunks.iter_mut() {
                for chunk in y.iter_mut() {
                    let vec = break_data_vecs_down_into_chunk_data(
                        &data,
                        chunk.chunk_pos,
                        max_chunk_size,
                    );
                    chunk.add_dense_layer_from_vecs(map_layer, vec);
                }
            }
            add_entities_to_layer(map_layer, chunks, entities, max_chunk_size);
        }
    }
}

pub fn break_layer_into_chunks<TileData>(
    tilemap_layer: &TilemapLayer<TileData>,
    max_chunk_size: UVec2,
) -> Vec<Vec<Chunk<TileData>>>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
{
    return match tilemap_layer {
        TilemapLayer::Sparse(data, map_size, entities) => {
            let mut chunks = break_hashmap_into_chunks(data, map_size.clone(), max_chunk_size);
            add_entities_to_layer(1u32, &mut chunks, entities, max_chunk_size);
            chunks
        }
        TilemapLayer::Dense(data, entities) => {
            let mut chunks = break_data_vecs_into_chunks(data, max_chunk_size);
            add_entities_to_layer(1u32, &mut chunks, entities, max_chunk_size);
            chunks
        }
    };
}

/// Function that breaks a [`HashMap<TilePos, TileData>`] into [`Vec<Vec<Chunk<TileData>>>`]
pub fn break_hashmap_into_chunks<TileData>(
    data: &HashMap<TilePos, TileData>,
    map_size: UVec2,
    max_chunk_size: UVec2,
) -> Vec<Vec<Chunk<TileData>>>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
{
    let mut chunks: Vec<Vec<Chunk<TileData>>> = vec![];
    // Get the chunks with the remainder for making chunks
    let max_chunks_floats = vec2(
        (f64::from(map_size.x) / f64::from(max_chunk_size.x)) as f32,
        (f64::from(map_size.y) / f64::from(max_chunk_size.y)) as f32,
    );

    // Get the amount of chunks we will have
    let max_chunks = UVec2::new(
        max_chunks_floats.x.ceil() as u32,
        max_chunks_floats.y.ceil() as u32,
    );

    for y in 0..max_chunks.y {
        let mut chunks_rows: Vec<Chunk<TileData>> = vec![];
        for x in 0..max_chunks.x {
            // Gets the actual chunk size of the given chunk
            let mut chunk_size = max_chunk_size;
            if y as f32 % max_chunks_floats.y != 0.0 {
                chunk_size.y =
                    ((max_chunks_floats.y - y as f32) * max_chunk_size.y as f32).ceil() as u32
            };
            if x as f32 % max_chunks_floats.x != 0.0 {
                chunk_size.x =
                    ((max_chunks_floats.x - x as f32) * max_chunk_size.x as f32).ceil() as u32
            };
            chunks_rows.push(Chunk::new_sparse(ChunkPos::new(x, y), chunk_size, None));
        }
        chunks.push(chunks_rows);
    }

    for (tile_pos, tile_data) in data.iter() {
        let chunk_pos = tile_pos.into_chunk_pos(max_chunk_size);
        chunks[chunk_pos.y() as usize][chunk_pos.x() as usize]
            .data
            .get_mut(&1u32)
            .unwrap()
            .set_tile_data(tile_pos.into_chunk_tile_pos(max_chunk_size), *tile_data);
    }

    chunks
}

/// Function that breaks a [`Vec<Vec<TileData>>`] into [`Vec<Vec<Chunk<TileData>>>`]
pub fn break_data_vecs_into_chunks<TileData>(
    data: &Vec<Vec<TileData>>,
    max_chunk_size: UVec2,
) -> Vec<Vec<Chunk<TileData>>>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
{
    let mut chunks: Vec<Vec<Chunk<TileData>>> = vec![];
    let map_x = data[0].len() as f32;
    let map_y = data.len() as f32;

    let x_chunk_amount = (map_x / max_chunk_size.x as f32).ceil() as u32;
    let y_chunk_amount = (map_y / max_chunk_size.y as f32).ceil() as u32;

    for y in 0..y_chunk_amount {
        let mut chunks_rows: Vec<Chunk<TileData>> = vec![];
        for x in 0..x_chunk_amount {
            let vec =
                break_data_vecs_down_into_chunk_data(&data, ChunkPos::new(x, y), max_chunk_size);
            let chunk = Chunk::<TileData>::new_dense_from_vecs(ChunkPos::new(x, y), &vec);
            chunks_rows.push(chunk);
        }
        chunks.push(chunks_rows);
    }

    chunks
}

/// Function that breaks a [`Vec<Vec<TileData>>`] down into a [`Vec<Vec<TileData>>`] of a specific chunks data
pub fn break_data_vecs_down_into_chunk_data<TileData>(
    data: &Vec<Vec<TileData>>,
    chunk_pos: ChunkPos,
    max_chunk_size: UVec2,
) -> Vec<Vec<TileData>>
where
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
{
    let amount_of_x_tiles_done = (chunk_pos.x() * max_chunk_size.x) as usize;
    let amount_of_y_tiles_done = (chunk_pos.y() * max_chunk_size.y) as usize;
    let mut vec: Vec<Vec<TileData>> = vec![];
    for y in amount_of_y_tiles_done..data.len() {
        if y >= (amount_of_y_tiles_done + max_chunk_size.y as usize) {
            continue;
        }
        let mut row_vec: Vec<TileData> = vec![];
        for x in amount_of_x_tiles_done..data[0].len() {
            if x >= (amount_of_x_tiles_done + max_chunk_size.x as usize) {
                continue;
            }
            row_vec.push(data[y][x]);
        }
        vec.push(row_vec);
    }
    vec
}

pub fn add_entities_to_layer<TileData>(
    map_layer: u32,
    chunks: &mut Vec<Vec<Chunk<TileData>>>,
    entities: &HashMap<TilePos, Entity>,
    max_chunk_size: UVec2,
) where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
{
    for (tile_pos, entity) in entities.iter() {
        let chunk_pos = tile_pos.into_chunk_pos(max_chunk_size);
        chunks[chunk_pos.y() as usize][chunk_pos.x() as usize]
            .data
            .get_mut(&map_layer)
            .unwrap()
            .set_tile_entity(tile_pos.into_chunk_tile_pos(max_chunk_size), *entity);
    }
}

#[cfg(test)]
mod tests {
    use crate as bevy_sparse_tilemap;

    use crate::tilemap_builder::tilemap_layer_builder::{break_hashmap_into_chunks, TilemapLayer};
    use crate::TilePos;
    use bevy::math::UVec2;
    use bevy::utils::HashMap;
    use bst_map_layer_derive::MapLayer;

    #[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
    struct TileData(u8);

    #[test]
    fn test_new_from_vecs() {
        // Tests basic i32
        let vecs = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let tilemap = TilemapLayer::new_dense_from_vecs(vecs);

        let TilemapLayer::Dense(data, ..) = tilemap else {
            panic!("Wrong type")
        };
        assert_eq!(data[0][0], 0);

        // Tests a custom struct as a TileData
        let vecs = vec![
            vec![TileData(0), TileData(1), TileData(2), TileData(3)],
            vec![TileData(4), TileData(5), TileData(6), TileData(7)],
            vec![TileData(8), TileData(9), TileData(10), TileData(11)],
        ];
        let tilemap = TilemapLayer::new_dense_from_vecs(vecs);
        let TilemapLayer::Dense(data, ..) = tilemap else {
            panic!("Wrong type")
        };
        assert_eq!(data[0][0], TileData(0));
        assert_eq!(data[2][3], TileData(11));
        // Tests tuples
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let tilemap = TilemapLayer::new_dense_from_vecs(vecs);
        let TilemapLayer::Dense(data, ..) = tilemap else {
            panic!("Wrong type")
        };
        assert_eq!(data[0][0], (0, 0));
        assert_eq!(data[2][2], (10, 1));
    }

    #[test]
    fn test_new_from_hashmap() {
        // Tests basic i32
        let mut hashmap: HashMap<TilePos, (u32, u32)> = HashMap::new();
        hashmap.insert(TilePos::new(0, 0), (0, 0));
        hashmap.insert(TilePos::new(31, 31), (31, 31));

        let tilemap = TilemapLayer::new_sparse_from_hashmap(32, 32, hashmap);

        let TilemapLayer::Sparse(data, size, ..) = tilemap else {
            panic!("Wrong type")
        };

        assert_eq!(size, UVec2::new(32, 32));

        assert_eq!(data.get(&TilePos::new(1, 1)).is_none(), true);
        assert_eq!(data.get(&TilePos::new(0, 0)).unwrap(), &(0, 0));
        assert_eq!(data.get(&TilePos::new(31, 31)).unwrap(), &(31, 31));
    }

    /// TilemapLayer breakdown
    use crate::map::chunk::ChunkPos;
    use crate::tilemap_builder::tilemap_layer_builder::break_data_vecs_down_into_chunk_data;

    #[test]
    fn test_vec_breakdown() {
        #[rustfmt::skip]
            let vecs = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0),(4, 0), (5, 0), (6, 0), (7, 0)],
            vec![(0, 1), (1, 1), (2, 1), (3, 1),(4, 1), (5, 1), (6, 1), (7, 1)],
            vec![(0, 2), (1, 2), (2, 2), (3, 2),(4, 2), (5, 2), (6, 2), (7, 2)],
            vec![(0, 3), (1, 3), (2, 3), (3, 3),(4, 3), (5, 3), (6, 3), (7, 3)],
            vec![(0, 4), (1, 4), (2, 4), (3, 4),(4, 4), (5, 4), (6, 4), (7, 4)],
            vec![(0, 5), (1, 5), (2, 5), (3, 5),(4, 5), (5, 5), (6, 5), (7, 5)],
            vec![(0, 6), (1, 6), (2, 6), (3, 6),(4, 6), (5, 6), (6, 6), (7, 6)],
            vec![(0, 7), (1, 7), (2, 7), (3, 7),(4, 7), (5, 7), (6, 7), (7, 7)],
            vec![(0, 8), (1, 8), (2, 8), (3, 8),(4, 8), (5, 8), (6, 8), (7, 8)]
        ];

        let max_chunk_size_x = 5;
        let max_chunk_size_y = 5;

        let zero_zero = break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(0, 0),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );

        assert_eq!(zero_zero[0][0], (0, 0));
        assert_eq!(zero_zero[0][4], (4, 0));
        assert_eq!(zero_zero[4][0], (0, 4));
        assert_eq!(zero_zero[4][4], (4, 4));

        let one_zero = break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(1, 0),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );

        assert_eq!(one_zero[0][0], (5, 0));
        assert_eq!(one_zero[0][2], (7, 0));
        assert_eq!(one_zero[4][0], (5, 4));
        assert_eq!(one_zero[4][2], (7, 4));

        let zero_one = break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(0, 1),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );
        assert_eq!(zero_one[0][0], (0, 5));
        assert_eq!(zero_one[0][4], (4, 5));
        assert_eq!(zero_one[3][0], (0, 8));
        assert_eq!(zero_one[3][4], (4, 8));
        let one_one = break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(1, 1),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );
        assert_eq!(one_one[0][0], (5, 5));
        assert_eq!(one_one[0][2], (7, 5));
        assert_eq!(one_one[3][0], (5, 8));
        assert_eq!(one_one[3][2], (7, 8));
    }

    #[derive(MapLayer)]
    enum MapLayers {
        Main,
    }

    #[test]
    fn test_hashmap_breakdown() {
        // Tests basic i32
        let mut hashmap: HashMap<TilePos, (u32, u32)> = HashMap::new();
        hashmap.insert(TilePos::new(0, 0), (0, 0));
        hashmap.insert(TilePos::new(5, 5), (5, 5));
        hashmap.insert(TilePos::new(1, 0), (1, 0));
        hashmap.insert(TilePos::new(0, 19), (0, 19));
        hashmap.insert(TilePos::new(31, 3), (31, 3));
        hashmap.insert(TilePos::new(12, 31), (12, 31));
        hashmap.insert(TilePos::new(10, 10), (10, 10));
        hashmap.insert(TilePos::new(15, 15), (15, 15));
        hashmap.insert(TilePos::new(27, 27), (27, 27));
        hashmap.insert(TilePos::new(31, 31), (31, 31));

        let mcs = UVec2::new(10, 10);

        let chunks = break_hashmap_into_chunks(&hashmap, UVec2::new(32, 32), mcs);

        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].len(), 4);

        assert_eq!(
            chunks[0][0]
                .get_tile_data(MapLayers::Main, TilePos::new(0, 0).into_chunk_tile_pos(mcs))
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunks[3][3]
                .get_tile_data(
                    MapLayers::Main,
                    TilePos::new(31, 31).into_chunk_tile_pos(mcs)
                )
                .unwrap(),
            (31, 31)
        );
        assert_eq!(
            chunks[0][0]
                .get_tile_data(MapLayers::Main, TilePos::new(0, 0).into_chunk_tile_pos(mcs))
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunks[0][0]
                .get_tile_data(MapLayers::Main, TilePos::new(0, 0).into_chunk_tile_pos(mcs))
                .unwrap(),
            (0, 0)
        );
    }
}
