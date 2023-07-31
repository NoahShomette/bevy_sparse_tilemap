//! Core Tilemap
//!
//! Tilemaps are a sparse-set representation of a tilemap. The minimum amount of data needed
//! saved into the Tilemaps chunks to be able to build out advanced functionality as needed.
//!
//!

use crate::map::chunk::Chunks;
use crate::{Chunk, ChunkPos, TilePos};
use bevy::math::UVec2;
use bevy::prelude::{BuildChildren, Commands, Component, Entity, Reflect};
use std::marker::PhantomData;

/// The data structure containing the minimum tilemap data needed for each tile as well as manages
/// chunk access and setup
///
/// Each tile should only contain the bare minimum data needed for you to figure out what it is. Any
/// data that is not the same for every single tile of that type should be stored as a component
/// on that tiles entity which is managed through the [`Chunk`]
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct Tilemap<TilemapMarker>
where
    TilemapMarker: Send + Sync + 'static,
{
    /// Struct containing [`Entity`] mappings to the [`Chunk`](super::chunk::Chunk)s that hold tile data
    chunks: Chunks,
    _marker: PhantomData<TilemapMarker>,
}

impl<TilemapMarker> Tilemap<TilemapMarker>
where
    TilemapMarker: Send + Sync + 'static,
{
    /// Spawns a new [`Tilemap`] and returns its [`Entity`] using the given vector of vectors of the given T.
    ///
    pub fn spawn_tilemap<T>(
        tilemap_tile_data: Vec<Vec<T>>,
        max_chunk_size: UVec2,
        commands: &mut Commands,
    ) -> Entity
    where
        T: Clone + Copy + Sized + Default + Send + Sync + 'static,
    {
        let mut chunk_entities: Vec<Vec<Entity>> = vec![];
        let map_x = tilemap_tile_data[0].len() as f32;
        let map_y = tilemap_tile_data.len() as f32;

        let x_chunk_amount = (map_x / max_chunk_size.x as f32).ceil() as u32;
        let y_chunk_amount = (map_y / max_chunk_size.y as f32).ceil() as u32;

        //TODO: We need to convert our tilemap_tile_data into a series of vecs of vecs for each chunk with only that chunks data in it
        
        for y in 0..y_chunk_amount {
            let mut x_vec: Vec<Entity> = vec![];
            for x in 0..x_chunk_amount {
                let amount_of_x_tiles_done = x * max_chunk_size.x;
                let amount_of_y_tiles_done = y * max_chunk_size.y;
                let mut vec:Vec<Vec<T>> = vec![];
                for (index, row) in tilemap_tile_data.iter().enumerate() {
                    let mut row_vec: Vec<T> = vec![];
                    for (index, tile) in row.iter().enumerate() {
                        if index < amount_of_x_tiles_done as usize {
                            continue
                        }
                        row_vec.push(*tile);
                    }
                    vec.push(row_vec);
                    if index < amount_of_y_tiles_done as usize {
                        continue
                    }
                }                
                
                let entity = commands
                    .spawn(Chunk::<T>::new_from_vecs(
                        ChunkPos::new(x, y),
                        vec,
                    ))
                    .id();
                x_vec.push(entity);
            }
            chunk_entities.push(x_vec);
        }

        let mut flattened_chunk_entities: Vec<Entity> = vec![];

        for chunk_entity in chunk_entities.iter_mut() {
            flattened_chunk_entities.extend(chunk_entity.iter().cloned())
        }

        let chunks = Chunks::new(
            Chunks::new_chunk_entity_grid(chunk_entities),
            max_chunk_size,
        );

        let tilemap_entity = commands
            .spawn(Tilemap::<TilemapMarker>::new(chunks))
            .push_children(flattened_chunk_entities.as_slice())
            .id();
        tilemap_entity
    }

    /// Creates a new [`Tilemap`] out of the given chunks struct
    pub fn new(chunks: Chunks) -> Tilemap<TilemapMarker> {
        Self {
            chunks,
            _marker: Default::default(),
        }
    }

    /// Gets the chunk entity that has the tile_info for the given TilePos
    pub fn get_chunk_for_tile_pos(&self, tile_pos: TilePos) -> Option<Entity> {
        self.chunks.get_chunk(tile_pos)
    }

    /// Returns the max size that a chunk can be
    pub fn get_chunks_max_size(&self) -> UVec2 {
        self.chunks.max_chunk_size()
    }
}

#[cfg(test)]
mod tests {
    use crate::map::tilemap::Tilemap;
}
