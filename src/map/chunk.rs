use crate::map::layer::{LayerChunkData, LayerTypeData, MapLayer};
use crate::TilePos;
use bevy::prelude::{Commands, Component, Entity, FromReflect, Reflect, ReflectComponent, UVec2};
use bevy::utils::hashbrown::HashMap;
use grid::{grid, Grid};
use std::hash::Hash;

/// The chunks of a tilemap
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chunks {
    /// A grid of [`Entity`] references pointing to that chunks entity
    chunks: Grid<Entity>,
    /// The max size that a chunk can be
    max_chunk_size: UVec2,
}

impl Chunks {
    /// Creates a new [`Grid<Entity>`] out of a vector of vectors of [`Entity`]
    pub fn new_chunk_entity_grid(tile_data: Vec<Vec<Entity>>) -> Grid<Entity> {
        let mut given_tile_count = 0u64;

        for tile_data in tile_data.iter() {
            given_tile_count += tile_data.len() as u64;
        }

        assert_eq!(
            (tile_data[0].len() * tile_data.len()) as u64,
            given_tile_count
        );

        let mut grid: Grid<Entity> =
            Grid::init(tile_data.len(), tile_data[0].len(), Entity::PLACEHOLDER);
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
        grid
    }

    /// Creates a new Chunks component
    pub fn new(chunk_entity_grid: Grid<Entity>, max_chunk_size: UVec2) -> Self {
        Self {
            chunks: chunk_entity_grid,
            max_chunk_size,
        }
    }

    /// Returns the max_chunk_size
    pub fn max_chunk_size(&self) -> UVec2 {
        self.max_chunk_size
    }

    /// Gets the tile info from a chunk based on a Tilemap TilePos. The chunks coords is
    /// grabbed and then taken from this
    pub fn get_chunk(&mut self, tile_pos: TilePos) -> Option<Entity> {
        let chunk_pos_x: usize = (tile_pos.x / self.max_chunk_size.x) as usize;
        let chunk_pos_y: usize = (tile_pos.y / self.max_chunk_size.y) as usize;
        self.chunks.get(chunk_pos_y, chunk_pos_x).cloned()
    }
}

/// The x and y position of a [`Chunk`] in the [`Tilemap`]
///
/// A [`TilePos`] can be converted into a [`ChunkPos`] using [`TilePos::into_chunk_pos`]
pub type ChunkPos = TilePos;

/// A tile position inside a [`Chunk`]
///
/// A [`TilePos`] can be converted into a [`ChunkTilePos`] using [`TilePos::into_chunk_tile_pos`]
pub type ChunkTilePos = TilePos;

/// A Chunk of a [`Tilemap`](super::Tilemap)
///
/// Contains all tile data as well as a hashmap that contains mapping to currently spawned tile entities
#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Chunk<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    pub chunk_pos: ChunkPos,
    /// Chunk tile data
    pub data: HashMap<u32, LayerChunkData<T>>,
}

impl<T> Default for Chunk<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    fn default() -> Self {
        Self {
            chunk_pos: Default::default(),
            data: HashMap::default(),
        }
    }
}

impl<T> Chunk<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    /// Creates a new chunk with every tile containing the same given tile_data
    pub fn new_uniform(
        chunk_pos: ChunkPos,
        chunk_size_x: usize,
        chunk_size_y: usize,
        tile_data: T,
    ) -> Chunk<T> {
        let mut hashmap: HashMap<u32, LayerChunkData<T>> = HashMap::new();
        hashmap.insert(
            1u32 << 0,
            LayerChunkData::new_dense_uniform_layer(chunk_size_x, chunk_size_y, tile_data),
        );
        Self {
            chunk_pos,
            data: hashmap,
        }
    }

    /// Creates a new chunk with every tile containing the default for T
    pub fn new_default(chunk_pos: ChunkPos, chunk_size_x: usize, chunk_size_y: usize) -> Chunk<T> {
        let mut hashmap: HashMap<u32, LayerChunkData<T>> = HashMap::new();
        hashmap.insert(
            1u32 << 0,
            LayerChunkData::new_dense_default_layer(chunk_size_x, chunk_size_y),
        );
        Self {
            chunk_pos,
            data: hashmap,
        }
    }

    /// Creates a new chunk from a vector of vectors of T
    ///
    /// # Panics
    /// - Panics if every row is not the same length
    pub fn new_from_vecs(chunk_pos: ChunkPos, tile_data: Vec<Vec<T>>) -> Chunk<T> {
        let mut hashmap: HashMap<u32, LayerChunkData<T>> = HashMap::new();
        hashmap.insert(
            1u32 << 0,
            LayerChunkData::new_dense_from_vecs_layer(tile_data),
        );
        Self {
            chunk_pos,
            data: hashmap,
        }
    }

    /// Sets the tile at the given [`TilePos`] to the given tile data.
    ///
    /// # Panics
    /// - If the [`ChunkTilePos`] does not exist in the [`Chunk`]
    pub fn set_tile_data(
        &mut self,
        map_layer: impl MapLayer,
        chunk_tile_pos: ChunkTilePos,
        tile_data: T,
    ) {
        if let Some(tiles) = self.data.get_mut(&map_layer.to_bits()) {
            tiles.set_tile_data(chunk_tile_pos, tile_data);
        };
    }

    /// Returns a clone of the TileData at the given [`ChunkTilePos`] if it exists
    pub fn get_tile_data(
        &self,
        map_layer: impl MapLayer,
        chunk_tile_pos: ChunkTilePos,
    ) -> Option<T> {
        self.data
            .get(&map_layer.to_bits())
            .and_then(|tiles| tiles.get_tile_data(chunk_tile_pos))
            .cloned()
    }

    pub fn get_tile_entity(&self, chunk_tile_pos: ChunkTilePos) -> Option<Entity> {
        self.get_tile_entity(chunk_tile_pos)
    }
}

/// Settings for the chunks in a [`Chunks`] object
pub struct ChunkSettings {
    /// The max size that a chunk can be
    max_chunk_size: UVec2,
}

impl ChunkSettings {
    pub fn max_chunk_size(&self) -> UVec2 {
        self.max_chunk_size
    }

    pub fn max_chunk_size_x(&self) -> u32 {
        self.max_chunk_size.x
    }

    pub fn max_chunk_size_y(&self) -> u32 {
        self.max_chunk_size.y
    }
}

#[cfg(test)]
mod tests {
    use crate::map::chunk::Chunk;
    use crate::map::ChunkPos;
    use crate::TilePos;
    use bevy_sparse_tilemap_derive::MapLayer;

    #[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
    struct TileData(u8);

    #[derive(MapLayer)]
    enum MapLayers {
        Main,
        Secondary,
    }

    #[test]
    fn test_new_from_vecs() {
        // Tests basic i32
        let vecs = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let chunk = Chunk::new_from_vecs(ChunkPos::new(0, 0), vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, TilePos { x: 0, y: 0 })
                .unwrap(),
            0
        );

        // Tests a custom struct as a TileData
        let vecs = vec![
            vec![TileData(0), TileData(1), TileData(2), TileData(3)],
            vec![TileData(4), TileData(5), TileData(6), TileData(7)],
            vec![TileData(8), TileData(9), TileData(10), TileData(11)],
        ];
        let chunk = Chunk::new_from_vecs(ChunkPos::new(0, 0), vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, TilePos { x: 0, y: 0 })
                .unwrap(),
            TileData(0)
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, TilePos { x: 3, y: 2 })
                .unwrap(),
            TileData(11)
        );

        // Tests tuples
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let chunk = Chunk::new_from_vecs(ChunkPos::new(0, 0), vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, TilePos { x: 0, y: 0 })
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, TilePos { x: 2, y: 2 })
                .unwrap(),
            (10, 1)
        );
    }

    #[test]
    #[should_panic]
    /// Panics because grid isn't uniform
    fn test_new_from_vecs_panic() {
        let vecs = vec![
            vec![(0), (1), (2)],
            vec![(4), (5), (6), (7)],
            vec![(8), (9), (10), (11)],
        ];
        let _ = Chunk::new_from_vecs(ChunkPos::new(0, 0), vecs);
    }

    #[test]
    fn test_chunks_tilepos_mapping() {
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let chunk = Chunk::new_from_vecs(ChunkPos::new(0, 0), vecs);
        assert_eq!(
            chunk.get_tile_data(TilePos { x: 3, y: 2 }).unwrap(),
            (11, 4)
        );
    }

    #[test]
    fn test_setting_chunk_tile() {
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let mut chunk = Chunk::new_from_vecs(ChunkPos::new(0, 0), vecs);
        chunk.set_tile_data(TilePos::new(0, 0), (50, 60));
        assert_eq!(
            chunk.get_tile_data(TilePos { x: 0, y: 0 }).unwrap(),
            (50, 60)
        );
    }
}
