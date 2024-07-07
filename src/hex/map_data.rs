use bevy::{
    math::{vec2, UVec2},
    prelude::Component,
    utils::hashbrown::HashMap,
};

#[cfg(feature = "reflect")]
use bevy::ecs::reflect::ReflectMapEntities;
#[cfg(feature = "reflect")]
use bevy::prelude::{Reflect, ReflectComponent};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::map::{
    chunk::{Chunk, ChunkLayerType, ChunkPos},
    MapData, MapLayer,
};

/// [`MapData`] implementation for a hexagonal map. Uses essentially the same logic as for a square map. Prior to map construction the map is in offset coordinates
#[derive(Default, Hash, Component)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Hash))]
pub struct HexMapData {
    /// The maximum size that chunk can be
    pub max_chunk_size: UVec2,
}

impl MapData for HexMapData {
    fn into_chunk_pos(&self, cell: lettuces::cell::Cell) -> ChunkPos {
        ChunkPos::new(
            cell.x / self.max_chunk_size.x as i32,
            cell.y / self.max_chunk_size.y as i32,
        )
    }

    fn max_chunk_size(&self) -> UVec2 {
        self.max_chunk_size
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
        chunk_settings: MapChunk::ChunkSettings,
    ) -> Vec<Vec<Chunk<MapChunk, TileData>>>
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
                    ChunkLayerType::Dense(vec),
                    chunk_settings,
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
        chunk_settings: MapChunk::ChunkSettings,
    ) -> Vec<Vec<Chunk<MapChunk, TileData>>>
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
                    ChunkLayerType::Sparse(HashMap::new()),
                    chunk_settings,
                ));
            }
            chunks.push(chunks_rows);
        }

        for (cell, tile_data) in data.iter() {
            let chunk_pos = self.into_chunk_pos(*cell);
            let chunk = &mut chunks[chunk_pos.y() as usize][chunk_pos.x() as usize];
            chunk.set_tile_data(
                map_layer.to_bits(),
                MapChunk::into_chunk_cell(*cell, &chunk.chunk_settings),
                *tile_data,
            );
        }

        chunks
    }
}
