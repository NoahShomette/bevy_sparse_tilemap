mod chunk_layer;
mod chunk_pos;
mod chunk_tile_pos;

pub use crate::map::chunk::chunk_layer::ChunkLayerData;
pub use crate::map::chunk::chunk_pos::ChunkPos;
pub use crate::map::chunk::chunk_tile_pos::ChunkTilePos;
use crate::map::MapLayer;
use crate::TilePos;
use bevy::prelude::{Component, Entity, Reflect, ReflectComponent, UVec2};
use bevy::utils::hashbrown::HashMap;
use grid::Grid;

/// The chunks of a tilemap
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chunks {
    /// A grid of [`Entity`] references pointing to that chunks entity
    chunk_entities: Grid<Entity>,
    /// The max size that a chunk can be
    max_chunk_size: UVec2,
}

impl Chunks {
    /// Creates a new [`Grid<Entity>`] out of a vector of vectors of [`Entity`]
    pub fn new_chunk_entity_grid(chunk_entities: Vec<Vec<Entity>>) -> Grid<Entity> {
        let mut counted_chunks = 0u64;

        for chunk_row in chunk_entities.iter() {
            counted_chunks += chunk_row.len() as u64;
        }

        assert_eq!(
            (chunk_entities[0].len() * chunk_entities.len()) as u64,
            counted_chunks
        );

        let mut grid: Grid<Entity> = Grid::init(
            chunk_entities.len(),
            chunk_entities[0].len(),
            Entity::PLACEHOLDER,
        );
        let mut current_x = 0usize;
        let mut current_y = 0usize;
        let row_length = chunk_entities[0].len();
        grid.fill_with(|| {
            let tile = chunk_entities[current_y][current_x];
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
            chunk_entities: chunk_entity_grid,
            max_chunk_size,
        }
    }

    /// Returns the max_chunk_size
    pub fn max_chunk_size(&self) -> UVec2 {
        self.max_chunk_size
    }

    /// Gets the chunk entity for the given [`ChunkPos`] if it exists
    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<Entity> {
        self.chunk_entities
            .get(chunk_pos.y() as usize, chunk_pos.x() as usize)
            .cloned()
    }

    /// Gets the chunk entity for the given [`TilePos`] if it exists
    pub fn get_chunk_from_tile_pos(&self, tile_pos: TilePos) -> Option<Entity> {
        let chunk_pos_x: usize = (tile_pos.x / self.max_chunk_size.x) as usize;
        let chunk_pos_y: usize = (tile_pos.y / self.max_chunk_size.y) as usize;
        self.chunk_entities.get(chunk_pos_y, chunk_pos_x).cloned()
    }

    /// Returns the x and y count of chunks
    pub fn chunk_counts(&self) -> UVec2 {
        UVec2::new(
            self.chunk_entities.size().1 as u32,
            self.chunk_entities.size().0 as u32,
        )
    }
}

/// A Chunk of a [`Tilemap`](super::Tilemap)
///
/// Contains all tile data as well as a hashmap that contains mapping to currently spawned tile entities
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Chunk<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    /// The position of the Chunk in the map
    pub chunk_pos: ChunkPos,
    /// Chunk tile data mapped to layers
    pub data: HashMap<u32, ChunkLayerData<T>>,
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
    ///
    /// # Note
    /// Automatically creates a new dense layer set to the first layer of a [`MapLayer`]
    pub fn new_uniform(
        chunk_pos: ChunkPos,
        chunk_size_x: usize,
        chunk_size_y: usize,
        tile_data: T,
    ) -> Chunk<T> {
        let mut hashmap: HashMap<u32, ChunkLayerData<T>> = HashMap::new();
        hashmap.insert(
            1u32,
            ChunkLayerData::new_dense_uniform_layer(chunk_size_x, chunk_size_y, tile_data),
        );
        Self {
            chunk_pos,
            data: hashmap,
        }
    }

    /// Creates a new chunk with every tile containing the default for T
    ///
    /// # Note
    /// Automatically creates a new dense layer set to the first layer of a [`MapLayer`]
    pub fn new_default(chunk_pos: ChunkPos, chunk_size_x: usize, chunk_size_y: usize) -> Chunk<T> {
        let mut hashmap: HashMap<u32, ChunkLayerData<T>> = HashMap::new();
        hashmap.insert(
            1u32,
            ChunkLayerData::new_dense_default_layer(chunk_size_x, chunk_size_y),
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
    ///
    /// # Note
    /// Automatically creates a new dense layer set to the first layer of a [`MapLayer`]
    pub fn new_dense_from_vecs(chunk_pos: ChunkPos, tile_data: &Vec<Vec<T>>) -> Chunk<T> {
        let mut hashmap: HashMap<u32, ChunkLayerData<T>> = HashMap::new();
        hashmap.insert(1u32, ChunkLayerData::new_dense_from_vecs_layer(tile_data));
        Self {
            chunk_pos,
            data: hashmap,
        }
    }

    /// Creates a new chunk with a Sparse layer
    pub fn new_sparse(
        chunk_pos: ChunkPos,
        chunk_size: UVec2,
        optional_tile_data: Option<HashMap<ChunkTilePos, T>>,
    ) -> Chunk<T> {
        let mut hashmap: HashMap<u32, ChunkLayerData<T>> = HashMap::new();
        match optional_tile_data {
            Some(data) => {
                hashmap.insert(
                    1u32,
                    ChunkLayerData::new_sparse_layer_from_data(data, chunk_size),
                );
            }
            _ => {
                hashmap.insert(1u32, ChunkLayerData::new_sparse_layer_empty(chunk_size));
            }
        };
        Self {
            chunk_pos,
            data: hashmap,
        }
    }

    /// Adds a new sparse layer to the chunk with the optional data.
    ///
    /// # Note
    /// - Overwrites the layer if it already exists
    pub fn add_sparse_layer(
        &mut self,
        map_layer: u32,
        optional_tile_data: Option<HashMap<ChunkTilePos, T>>,
    ) {
        match optional_tile_data {
            None => {
                self.data.insert(
                    map_layer,
                    ChunkLayerData::new_sparse_layer_empty(self.get_chunk_dimensions()),
                );
            }
            Some(data) => {
                self.data.insert(
                    map_layer,
                    ChunkLayerData::new_sparse_layer_from_data(data, self.get_chunk_dimensions()),
                );
            }
        };
    }

    /// Adds a new dense layer to the chunk and fills the layer with the default for T.
    ///
    /// # Note
    /// - Overwrites the layer if it already exists
    pub fn add_dense_layer_default(&mut self, map_layer: u32) {
        self.data.insert(
            map_layer,
            ChunkLayerData::new_dense_default_layer(
                self.get_chunk_dimensions().x as usize,
                self.get_chunk_dimensions().y as usize,
            ),
        );
    }

    /// Adds a new dense layer to the chunk and fills the layer with given tile_data.
    ///
    /// # Note
    /// - Overwrites the layer if it already exists
    pub fn add_dense_layer_uniform(&mut self, map_layer: u32, tile_data: T) {
        self.data.insert(
            map_layer,
            ChunkLayerData::new_dense_uniform_layer(
                self.get_chunk_dimensions().x as usize,
                self.get_chunk_dimensions().y as usize,
                tile_data,
            ),
        );
    }

    /// Adds a new dense layer to the chunk and fills the layer with given tile_data.
    ///
    /// # Note
    /// - Overwrites the layer if it already exists
    pub fn add_dense_layer_from_vecs(&mut self, map_layer: u32, tile_data: Vec<Vec<T>>) {
        self.data.insert(
            map_layer,
            ChunkLayerData::new_dense_from_vecs_layer(&tile_data),
        );
    }
}

impl<T> Chunk<T>
where
    T: Clone + Copy + Sized + Default + Send + Sync,
{
    /// Returns the actual dimensions for the given [`MapLayer`] in the [`Chunk`].
    ///
    /// # Panics
    /// - If the [`MapLayer`] does not exist in the chunk
    pub fn get_chunk_dimensions(&self) -> UVec2 {
        if let Some(tiles) = self.data.get(&1u32) {
            return tiles.get_chunk_dimensions();
        } else {
            panic!("MapLayer does not exist in chunk")
        }
    }

    /// Sets the tile at the given [`TilePos`] to the given tile data.
    ///
    /// # Panics
    /// - If the [`ChunkTilePos`] does not exist in the [`Chunk`]
    /// - If the [`MapLayer`] does not exist in the chunk
    pub fn set_tile_data(
        &mut self,
        map_layer: impl MapLayer,
        chunk_tile_pos: ChunkTilePos,
        tile_data: T,
    ) {
        if let Some(tiles) = self.data.get_mut(&map_layer.to_bits()) {
            tiles.set_tile_data(chunk_tile_pos, tile_data);
        } else {
            panic!("MapLayer does not exist in chunk")
        }
    }

    /// Returns a clone of the TileData at the given [`ChunkTilePos`] if it exists
    ///
    /// # Panics
    /// - If the [`MapLayer`] does not exist in the chunk
    pub fn get_tile_data(
        &self,
        map_layer: impl MapLayer,
        chunk_tile_pos: ChunkTilePos,
    ) -> Option<T> {
        self.data
            .get(&map_layer.to_bits())
            .expect("MapLayer does not exist in chunk")
            .get_tile_data(chunk_tile_pos)
            .cloned()
    }

    pub fn get_tile_entity(
        &self,
        map_layer: impl MapLayer,
        chunk_tile_pos: ChunkTilePos,
    ) -> Option<Entity> {
        self.data
            .get(&map_layer.to_bits())
            .expect("MapLayer does not exist in chunk")
            .get_tile_entity(chunk_tile_pos)
    }

    /// Sets the [`Entity`] for the given [`ChunkTilePos`] to the given Entity.
    pub fn set_tile_entity(
        &mut self,
        map_layer: impl MapLayer,
        chunk_tile_pos: ChunkTilePos,
        entity: Entity,
    ) {
        self.data
            .get_mut(&map_layer.to_bits())
            .expect("MapLayer does not exist in chunk")
            .set_tile_entity(chunk_tile_pos, entity);
    }
}

/// Settings for the chunks in a [`Chunks`] object
pub struct ChunkSettings {
    /// The max size that a chunk can be
    pub max_chunk_size: UVec2,
}

impl ChunkSettings {
    pub fn new(x: u32, y: u32) -> ChunkSettings {
        Self {
            max_chunk_size: UVec2::new(x, y),
        }
    }

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
    use crate as bevy_sparse_tilemap;
    use crate::{
        map::chunk::chunk_pos::ChunkPos, map::chunk::chunk_tile_pos::ChunkTilePos,
        map::chunk::Chunk,
    };
    use bevy::utils::HashMap;
    use bst_map_layer_derive::MapLayer;

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
        let chunk = Chunk::new_dense_from_vecs(ChunkPos::new(0, 0), &vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(0, 0))
                .unwrap(),
            0
        );

        // Tests a custom struct as a TileData
        let vecs = vec![
            vec![TileData(0), TileData(1), TileData(2), TileData(3)],
            vec![TileData(4), TileData(5), TileData(6), TileData(7)],
            vec![TileData(8), TileData(9), TileData(10), TileData(11)],
        ];
        let chunk = Chunk::new_dense_from_vecs(ChunkPos::new(0, 0), &vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(0, 0))
                .unwrap(),
            TileData(0)
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(3, 2))
                .unwrap(),
            TileData(11)
        );

        // Tests tuples
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let chunk = Chunk::new_dense_from_vecs(ChunkPos::new(0, 0), &vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(0, 0))
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(2, 2))
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
        let _ = Chunk::new_dense_from_vecs(ChunkPos::new(0, 0), &vecs);
    }

    #[test]
    fn test_chunks_tilepos_mapping() {
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let chunk = Chunk::new_dense_from_vecs(ChunkPos::new(0, 0), &vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(3, 2))
                .unwrap(),
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
        let mut chunk = Chunk::new_dense_from_vecs(ChunkPos::new(0, 0), &vecs);
        chunk.set_tile_data(MapLayers::Main, ChunkTilePos::new(0, 0), (50, 60));
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkTilePos::new(0, 0))
                .unwrap(),
            (50, 60)
        );
    }

    #[test]
    fn test_adding_sparse_layer() {
        let mut hashmap: HashMap<ChunkTilePos, (u32, u32)> = HashMap::new();
        hashmap.insert(ChunkTilePos::new(0, 0), (50, 60));
        let mut chunk = Chunk::new_uniform(ChunkPos::new(0, 0), 30, 30, (0u32, 0u32));
        chunk.add_sparse_layer(MapLayers::Secondary.to_bits(), Some(hashmap));
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Secondary, ChunkTilePos::new(0, 0))
                .unwrap(),
            (50, 60)
        );
    }

    #[test]
    fn test_adding_dense_layer() {
        let mut chunk = Chunk::new_uniform(ChunkPos::new(0, 0), 30, 30, (0u32, 0u32));

        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];

        chunk.add_dense_layer_from_vecs(MapLayers::Secondary.to_bits(), vecs);
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Secondary, ChunkTilePos::new(3, 2))
                .unwrap(),
            (11, 4)
        );
    }
}
