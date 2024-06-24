use bevy::{
    math::{vec2, UVec2},
    utils::hashbrown::HashMap,
};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::map::{
    chunk::{Chunk, ChunkPos, LayerType},
    MapData, MapLayer,
};

#[derive(Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", reflect(Hash))]
pub struct SquareMapDataConversionSettings {
    pub max_chunk_dimensions: UVec2,
}

impl Default for SquareMapDataConversionSettings {
    fn default() -> Self {
        Self {
            max_chunk_dimensions: UVec2 { x: 10, y: 10 },
        }
    }
}

#[derive(Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Hash))]
pub struct SquareMapData {
    pub conversion_settings: SquareMapDataConversionSettings,
}

impl MapData for SquareMapData {
    type ChunkPosConversionInfo = SquareMapDataConversionSettings;

    fn into_chunk_pos(
        cell: lettuces::cell::Cell,
        conversion_settings: &Self::ChunkPosConversionInfo,
    ) -> crate::map::chunk::ChunkPos {
        ChunkPos::new(
            cell.x / conversion_settings.max_chunk_dimensions.x as i32,
            cell.y / conversion_settings.max_chunk_dimensions.y as i32,
        )
    }

    fn conversion_info(&self) -> &Self::ChunkPosConversionInfo {
        &self.conversion_settings
    }

    fn break_data_vecs_down_into_chunk_data<TileData>(
        &self,
        data: &Vec<Vec<TileData>>,
        chunk_pos: ChunkPos,
        max_chunk_size: UVec2,
    ) -> Vec<Vec<TileData>>
    where
        TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    {
        let amount_of_x_tiles_done = (chunk_pos.x() * max_chunk_size.x as i32) as usize;
        let amount_of_y_tiles_done = (chunk_pos.y() * max_chunk_size.y as i32) as usize;
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

    fn break_data_vecs_into_chunks<TileData, MapChunk>(
        &self,
        data: &Vec<Vec<TileData>>,
        max_chunk_size: UVec2,
        chunk_conversion_settings: MapChunk::ConversionInfo,
        map_settings: MapChunk::MapSettings,
    ) -> Vec<Vec<crate::map::chunk::Chunk<MapChunk, TileData>>>
    where
        TileData: std::hash::Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
        MapChunk: crate::map::chunk::ChunkLayer<TileData> + Send + Sync + 'static + Default,
    {
        let mut chunks: Vec<Vec<Chunk<MapChunk, TileData>>> = vec![];
        let map_x = data[0].len() as f32;
        let map_y = data.len() as f32;

        let chunks_on_x = (map_x / max_chunk_size.x as f32).ceil() as i32;
        let chunks_on_y = (map_y / max_chunk_size.y as f32).ceil() as i32;

        for y in 0..chunks_on_y {
            let mut chunks_rows: Vec<Chunk<MapChunk, TileData>> = vec![];
            for x in 0..chunks_on_x {
                let vec = self.break_data_vecs_down_into_chunk_data(
                    &data,
                    ChunkPos::new(x, y),
                    max_chunk_size,
                );
                let chunk = Chunk::<MapChunk, TileData>::new(
                    ChunkPos::new(x, y),
                    UVec2::new(vec.len() as u32, vec[0].len() as u32),
                    LayerType::Dense(vec),
                    chunk_conversion_settings,
                    map_settings,
                );
                chunks_rows.push(chunk);
            }
            chunks.push(chunks_rows);
        }

        chunks
    }

    fn break_hashmap_into_chunks<TileData, MapChunk>(
        &self,
        map_layer: impl MapLayer,
        data: &bevy::utils::HashMap<lettuces::cell::Cell, TileData>,
        map_size: UVec2,
        max_chunk_size: UVec2,
        chunk_conversion_settings: MapChunk::ConversionInfo,
        map_settings: MapChunk::MapSettings,
    ) -> Vec<Vec<crate::map::chunk::Chunk<MapChunk, TileData>>>
    where
        TileData: std::hash::Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
        MapChunk: crate::map::chunk::ChunkLayer<TileData> + Send + Sync + 'static + Default,
    {
        let mut chunks: Vec<Vec<Chunk<MapChunk, TileData>>> = vec![];
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

        for y in 0..max_chunks.y as i32 {
            let mut chunks_rows: Vec<Chunk<MapChunk, TileData>> = vec![];
            for x in 0..max_chunks.x as i32 {
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
                chunks_rows.push(Chunk::new(
                    ChunkPos::new(x, y),
                    chunk_size,
                    LayerType::Sparse(HashMap::new()),
                    chunk_conversion_settings,
                    map_settings,
                ));
            }
            chunks.push(chunks_rows);
        }

        for (cell, tile_data) in data.iter() {
            let chunk_pos = Self::into_chunk_pos(*cell, &self.conversion_settings);
            let chunk = &mut chunks[chunk_pos.y() as usize][chunk_pos.x() as usize];
            chunk.set_tile_data(
                map_layer.to_bits(),
                MapChunk::into_chunk_cell(*cell, &chunk.cell_conversion_settings),
                *tile_data,
            );
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use crate as bevy_sparse_tilemap;
    use crate::map::MapData;
    use crate::square::map_chunk_layer::{SquareChunkLayer, SquareChunkLayerConversionSettings};
    use crate::square::map_data::{SquareMapData, SquareMapDataConversionSettings};

    use crate::tilemap_builder::tilemap_layer_builder::TilemapLayer;
    use bevy::math::UVec2;
    use bevy::utils::HashMap;
    use bst_map_layer_derive::MapLayer;
    use lettuces::cell::Cell;

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
        let mut hashmap: HashMap<Cell, (u32, u32)> = HashMap::new();
        hashmap.insert(Cell::new(0, 0), (0, 0));
        hashmap.insert(Cell::new(31, 31), (31, 31));

        let tilemap = TilemapLayer::new_sparse_from_hashmap(32, 32, hashmap);

        let TilemapLayer::Sparse(data, size, ..) = tilemap else {
            panic!("Wrong type")
        };

        assert_eq!(size, UVec2::new(32, 32));

        assert_eq!(data.get(&Cell::new(1, 1)).is_none(), true);
        assert_eq!(data.get(&Cell::new(0, 0)).unwrap(), &(0, 0));
        assert_eq!(data.get(&Cell::new(31, 31)).unwrap(), &(31, 31));
    }

    /// TilemapLayer breakdown
    use crate::map::chunk::ChunkPos;

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

        let map_data = SquareMapData {
            conversion_settings: SquareMapDataConversionSettings {
                max_chunk_dimensions: UVec2 { x: 5, y: 5 },
            },
        };

        let zero_zero = map_data.break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(0, 0),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );

        assert_eq!(zero_zero[0][0], (0, 0));
        assert_eq!(zero_zero[0][4], (4, 0));
        assert_eq!(zero_zero[4][0], (0, 4));
        assert_eq!(zero_zero[4][4], (4, 4));

        let one_zero = map_data.break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(1, 0),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );

        assert_eq!(one_zero[0][0], (5, 0));
        assert_eq!(one_zero[0][2], (7, 0));
        assert_eq!(one_zero[4][0], (5, 4));
        assert_eq!(one_zero[4][2], (7, 4));

        let zero_one = map_data.break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(0, 1),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );
        assert_eq!(zero_one[0][0], (0, 5));
        assert_eq!(zero_one[0][4], (4, 5));
        assert_eq!(zero_one[3][0], (0, 8));
        assert_eq!(zero_one[3][4], (4, 8));
        let one_one = map_data.break_data_vecs_down_into_chunk_data(
            &vecs,
            ChunkPos::new(1, 1),
            UVec2::new(max_chunk_size_x, max_chunk_size_y),
        );
        assert_eq!(one_one[0][0], (5, 5));
        assert_eq!(one_one[0][2], (7, 5));
        assert_eq!(one_one[3][0], (5, 8));
        assert_eq!(one_one[3][2], (7, 8));
    }

    #[derive(MapLayer, Default)]
    enum MapLayers {
        #[default]
        Main,
    }

    #[test]
    fn test_hashmap_breakdown() {
        let map_data = SquareMapData {
            conversion_settings: SquareMapDataConversionSettings {
                max_chunk_dimensions: UVec2 { x: 10, y: 10 },
            },
        };

        let chunk_conversion_settings = SquareChunkLayerConversionSettings {
            max_chunk_size: UVec2 { x: 10, y: 10 },
        };

        // Tests basic i32
        let mut hashmap: HashMap<Cell, (u32, u32)> = HashMap::new();
        hashmap.insert(Cell::new(0, 0), (0, 0));
        hashmap.insert(Cell::new(5, 5), (5, 5));
        hashmap.insert(Cell::new(1, 0), (1, 0));
        hashmap.insert(Cell::new(0, 19), (0, 19));
        hashmap.insert(Cell::new(31, 3), (31, 3));
        hashmap.insert(Cell::new(12, 31), (12, 31));
        hashmap.insert(Cell::new(10, 10), (10, 10));
        hashmap.insert(Cell::new(15, 15), (15, 15));
        hashmap.insert(Cell::new(27, 27), (27, 27));
        hashmap.insert(Cell::new(31, 31), (31, 31));

        let mcs = UVec2::new(10, 10);

        let chunks: Vec<Vec<crate::map::chunk::Chunk<SquareChunkLayer<(u32, u32)>, (u32, u32)>>> =
            map_data.break_hashmap_into_chunks(
                MapLayers::Main,
                &hashmap,
                UVec2::new(32, 32),
                mcs,
                chunk_conversion_settings,
                (),
            );

        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].len(), 4);

        assert_eq!(
            chunks[0][0]
                .get_tile_data_from_cell(MapLayers::Main, Cell::new(0, 0))
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunks[3][3]
                .get_tile_data_from_cell(MapLayers::Main, Cell::new(31, 31))
                .unwrap(),
            (31, 31)
        );
        assert_eq!(
            chunks[0][0]
                .get_tile_data_from_cell(MapLayers::Main, Cell::new(0, 0))
                .unwrap(),
            (0, 0)
        );
        assert_eq!(
            chunks[0][0]
                .get_tile_data_from_cell(MapLayers::Main, Cell::new(0, 0))
                .unwrap(),
            (0, 0)
        );
    }
}
