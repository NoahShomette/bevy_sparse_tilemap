mod chunk_cell;
mod chunk_pos;
mod layer_data;

pub use crate::map::chunk::chunk_cell::ChunkCell;
pub use crate::map::chunk::chunk_pos::ChunkPos;
use crate::map::MapLayer;
use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::prelude::{Component, Entity, UVec2};
use bevy::utils::hashbrown::HashMap;
pub use layer_data::{ChunkLayer, LayerType};
use lettuces::cell::Cell;
use lettuces::storage::grid::Grid;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};

/// The chunks of a tilemap
#[derive(Clone, Component, Hash, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Hash, MapEntities))]
pub struct Chunks {
    /// A grid of [`Entity`] references pointing to that chunks entity
    chunk_entities: Grid<Entity>,
    /// The max size that a chunk can be
    max_chunk_size: UVec2,
}

impl MapEntities for Chunks {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for tile_entity in self.chunk_entities.iter_mut() {
            *tile_entity = entity_mapper.map_entity(*tile_entity);
        }
    }
}

impl Default for Chunks {
    fn default() -> Self {
        Self {
            chunk_entities: Grid::<Entity>::init(0, 0, Entity::PLACEHOLDER),
            max_chunk_size: Default::default(),
        }
    }
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
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component, Hash, MapEntities))]
pub struct Chunk<MapChunk, TileData>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync,
    MapChunk: ChunkLayer<TileData> + Send + Sync + Default,
{
    /// The position of the Chunk in the map
    pub chunk_pos: ChunkPos,
    /// Chunk tile data mapped to layers
    pub data: HashMap<u32, MapChunk>,
    /// Settings related to the chunk
    pub chunk_settings: MapChunk::ChunkSettings,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    ph: PhantomData<TileData>,
}

impl<MapChunk, TileData> MapEntities for Chunk<MapChunk, TileData>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync,
    MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default,
{
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for datum in self.data.iter_mut() {
            datum.1.map_entities(entity_mapper);
        }
    }
}

impl<MapChunk, TileData> Hash for Chunk<MapChunk, TileData>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync,
    MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default,
{
    fn hash<H: Hasher>(&self, h: &mut H) {
        let mut pairs: Vec<_> = self.data.iter().collect();
        pairs.sort_by_key(|i| i.0);
        Hash::hash(&pairs, h);
        Hash::hash(&self.chunk_pos, h);
    }
}

impl<MapChunk, TileData> Default for Chunk<MapChunk, TileData>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync,
    MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default,
{
    fn default() -> Self {
        Self {
            chunk_pos: Default::default(),
            data: HashMap::default(),
            chunk_settings: MapChunk::ChunkSettings::default(),
            ph: Default::default(),
        }
    }
}

impl<MapChunk, TileData> Chunk<MapChunk, TileData>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync,
    MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default,
{
    /// Creates a new chunk with the given data. chunk size represents the actual size of the chunk object.
    pub fn new(
        chunk_pos: ChunkPos,
        chunk_size: UVec2,
        tile_data: LayerType<TileData>,
        chunk_settings: MapChunk::ChunkSettings,
    ) -> Chunk<MapChunk, TileData> {
        let mut hashmap = HashMap::new();
        hashmap.insert(1u32, MapChunk::new(tile_data, chunk_size, &chunk_settings));
        Self {
            chunk_pos,
            data: hashmap,
            chunk_settings,
            ph: Default::default(),
        }
    }

    /// Adds a new layer to the chunk with the given data.
    ///
    /// # Note
    /// - Overwrites the layer if it already exists
    pub fn add_layer(&mut self, map_layer: u32, tile_data: LayerType<TileData>) {
        self.data.insert(
            map_layer,
            MapChunk::new(tile_data, self.get_chunk_dimensions(), &self.chunk_settings),
        );
    }
}

impl<MapChunk, TileData> Chunk<MapChunk, TileData>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync,
    MapChunk: ChunkLayer<TileData> + Send + Sync + 'static + Default,
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
    pub fn set_tile_data_from_cell(&mut self, map_layer: u32, cell: Cell, tile_data: TileData) {
        self.set_tile_data(
            map_layer,
            MapChunk::into_chunk_cell(cell, &self.chunk_settings),
            tile_data,
        )
    }

    /// Sets the tile at the given [`TilePos`] to the given tile data.
    ///
    /// # Panics
    /// - If the [`ChunkTilePos`] does not exist in the [`Chunk`]
    /// - If the [`MapLayer`] does not exist in the chunk
    pub fn set_tile_data(&mut self, map_layer: u32, chunk_cell: ChunkCell, tile_data: TileData) {
        if let Some(tiles) = self.data.get_mut(&map_layer) {
            tiles.set_tile_data(chunk_cell, tile_data);
        } else {
            panic!("MapLayer does not exist in chunk")
        }
    }

    /// Returns a clone of the TileData at the given world [`Cell`] if it exists in this chunk
    ///
    /// # Panics
    /// - If the [`MapLayer`] does not exist in the chunk
    pub fn get_tile_data_from_cell(
        &self,
        map_layer: impl MapLayer,
        cell: Cell,
    ) -> Option<TileData> {
        self.get_tile_data(
            map_layer,
            MapChunk::into_chunk_cell(cell, &self.chunk_settings),
        )
    }

    /// Returns a clone of the TileData at the given [`ChunkTilePos`] if it exists
    ///
    /// # Panics
    /// - If the [`MapLayer`] does not exist in the chunk
    pub fn get_tile_data(
        &self,
        map_layer: impl MapLayer,
        chunk_cell: ChunkCell,
    ) -> Option<TileData> {
        self.data
            .get(&map_layer.to_bits())
            .expect("MapLayer does not exist in chunk")
            .get_tile_data(chunk_cell)
            .cloned()
    }

    pub fn get_tile_entity_from_cell(
        &self,
        map_layer: impl MapLayer,
        cell: Cell,
    ) -> Option<Entity> {
        self.get_tile_entity(
            map_layer,
            MapChunk::into_chunk_cell(cell, &self.chunk_settings),
        )
    }

    pub fn get_tile_entity(
        &self,
        map_layer: impl MapLayer,
        chunk_cell: ChunkCell,
    ) -> Option<Entity> {
        self.data
            .get(&map_layer.to_bits())
            .expect("MapLayer does not exist in chunk")
            .get_tile_entity(chunk_cell)
    }

    /// Sets the [`Entity`] for the given [`ChunkTilePos`] to the given Entity.
    pub fn set_tile_entity_from_cell(&mut self, map_layer: u32, cell: Cell, entity: Entity) {
        self.set_tile_entity(
            map_layer,
            MapChunk::into_chunk_cell(cell, &self.chunk_settings),
            entity,
        )
    }

    /// Sets the [`Entity`] for the given [`ChunkTilePos`] to the given Entity.
    pub fn set_tile_entity(&mut self, map_layer: u32, chunk_cell: ChunkCell, entity: Entity) {
        self.data
            .get_mut(&map_layer)
            .expect("MapLayer does not exist in chunk")
            .set_tile_entity(chunk_cell, entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::square::map_chunk_layer::{SquareChunkLayer, SquareChunkSettings};
    use crate::{self as bevy_sparse_tilemap};
    use crate::{
        map::chunk::chunk_cell::ChunkCell, map::chunk::chunk_pos::ChunkPos, map::chunk::Chunk,
    };
    use bevy::math::UVec2;
    use bevy::utils::hashbrown::HashMap;
    use bst_map_layer_derive::MapLayer;

    #[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash)]
    struct TileData(u8);

    #[derive(MapLayer, Default)]
    enum MapLayers {
        #[default]
        Main,
        Secondary,
    }

    #[test]
    fn test_new_from_vecs() {
        // Tests basic i32
        let vecs = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let chunk: Chunk<SquareChunkLayer<i32>, i32> = Chunk::new(
            ChunkPos::new(0, 0),
            UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Dense(vecs),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(0, 0))
                .unwrap(),
            0
        );

        // Tests a custom struct as a TileData
        let vecs = vec![
            vec![TileData(0), TileData(1), TileData(2), TileData(3)],
            vec![TileData(4), TileData(5), TileData(6), TileData(7)],
            vec![TileData(8), TileData(9), TileData(10), TileData(11)],
        ];
        let chunk: Chunk<SquareChunkLayer<TileData>, TileData> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Dense(vecs),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(0, 0))
                .unwrap(),
            TileData(0)
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(3, 2))
                .unwrap(),
            TileData(11)
        );

        // Tests tuples
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let chunk: Chunk<SquareChunkLayer<(i32, i32)>, (i32, i32)> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Dense(vecs),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(0, 0))
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(2, 2))
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
        let _chunk: Chunk<SquareChunkLayer<i32>, i32> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Dense(vecs),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
    }

    #[test]
    fn test_chunks_tilepos_mapping() {
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];
        let chunk: Chunk<SquareChunkLayer<(i32, i32)>, (i32, i32)> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Dense(vecs),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(3, 2))
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
        let mut chunk: Chunk<SquareChunkLayer<(i32, i32)>, (i32, i32)> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Dense(vecs),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        chunk.set_tile_data(MapLayers::Main.to_bits(), ChunkCell::new(0, 0), (50, 60));
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Main, ChunkCell::new(0, 0))
                .unwrap(),
            (50, 60)
        );
    }

    #[test]
    fn test_adding_sparse_layer() {
        let mut hashmap: HashMap<ChunkCell, (u32, u32)> = HashMap::new();
        hashmap.insert(ChunkCell::new(0, 0), (50, 60));
        let mut chunk: Chunk<SquareChunkLayer<(u32, u32)>, (u32, u32)> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Sparse(HashMap::new()),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        chunk.add_layer(
            MapLayers::Secondary.to_bits(),
            crate::map::chunk::LayerType::Sparse(hashmap),
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Secondary, ChunkCell::new(0, 0))
                .unwrap(),
            (50, 60)
        );
    }

    #[test]
    fn test_adding_dense_layer() {
        let mut chunk: Chunk<SquareChunkLayer<(i32, i32)>, (i32, i32)> = Chunk::new(
            ChunkPos::new(0, 0),
            bevy::math::UVec2 { x: 2, y: 2 },
            crate::map::chunk::LayerType::Sparse(HashMap::new()),
            SquareChunkSettings {
                max_chunk_size: UVec2 { x: 2, y: 2 },
            },
        );
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4)],
        ];

        chunk.add_layer(
            MapLayers::Secondary.to_bits(),
            crate::map::chunk::LayerType::Dense(vecs),
        );
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Secondary, ChunkCell::new(3, 2))
                .unwrap(),
            (11, 4)
        );
    }

    #[cfg(feature = "reflect")]
    mod reflect_test {
        use crate::square::map_chunk_layer::{
            SquareChunkLayer, SquareChunkSettings, SquareChunkLayerData,
        };
        use crate::square::map_data::SquareMapDataConversionSettings;
        use crate::{self as bevy_sparse_tilemap};
        use crate::{
            map::chunk::chunk_cell::ChunkCell, map::chunk::chunk_pos::ChunkPos, map::chunk::Chunk,
        };
        use bevy::math::UVec2;
        use bevy::prelude::{Entity, FromReflect, Reflect};
        use bevy::reflect::erased_serde::__private::serde::de::DeserializeSeed;
        use bevy::reflect::serde::{ReflectSerializer, UntypedReflectDeserializer};
        use bevy::reflect::TypeRegistry;
        use bevy::utils::hashbrown::HashMap;
        use bst_map_layer_derive::MapLayer;
        use lettuces::cell::Cell;
        use lettuces::storage::grid::Grid;

        #[test]
        fn test_hashing_chunk() {
            let chunk: Chunk<SquareChunkLayer<(u32, u32)>, (u32, u32)> = Chunk::new(
                ChunkPos::new(0, 0),
                bevy::math::UVec2 { x: 2, y: 2 },
                crate::map::chunk::LayerType::Sparse::<(u32, u32)>(HashMap::new()),
                SquareChunkSettings {
                    max_chunk_size: UVec2 { x: 2, y: 2 },
                },
            );
            let mut registry = TypeRegistry::default();
            registry.register::<Chunk<SquareChunkLayer<(u32, u32)>, (u32, u32)>>();
            registry.register::<ChunkPos>();
            registry.register::<Cell>();
            registry.register::<HashMap<u32, SquareChunkLayer<(u32, u32)>>>();
            registry.register::<HashMap<u64, Entity>>();
            registry.register::<HashMap<u64, (u32, u32)>>();
            registry.register::<SquareChunkLayer<(u32, u32)>>();
            registry.register::<SquareChunkLayerConversionSettings>();
            registry.register::<SquareMapDataConversionSettings>();
            registry.register::<SquareChunkLayerData<(u32, u32)>>();
            registry.register::<Grid<(u32, u32)>>();
            registry.register::<Vec<(u32, u32)>>();
            registry.register::<(u32, u32)>();
            registry.register::<Entity>();
            registry.register::<UVec2>();

            // Serialize
            let reflect_serializer = ReflectSerializer::new(&chunk, &registry);
            let serialized_value: String = ron::to_string(&reflect_serializer).unwrap();

            // Deserialize
            let reflect_deserializer = UntypedReflectDeserializer::new(&registry);
            let deserialized_value: Box<dyn Reflect> = reflect_deserializer
                .deserialize(&mut ron::Deserializer::from_str(&serialized_value).unwrap())
                .unwrap();

            // Convert
            let converted_value =
                <Chunk<SquareChunkLayer<(u32, u32)>, (u32, u32)> as FromReflect>::from_reflect(
                    &*deserialized_value,
                )
                .unwrap();

            assert_eq!(converted_value.chunk_pos, ChunkPos::new(0, 0));
        }
    }
}
