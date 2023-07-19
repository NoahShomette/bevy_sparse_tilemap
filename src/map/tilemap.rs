//! Core Tilemap
//!
//! Tilemaps are a sparse-set representation of a tilemap. The minimum amount of data needed
//! saved into the Tilemaps chunks to be able to build out advanced functionality as needed.
//!
//!

use crate::map::chunk::Chunks;
use crate::{Chunk, ChunkPos, TilePos};
use bevy::math::UVec2;
use bevy::prelude::{
    BuildChildren, Commands, Component, Entity, FromReflect, Reflect, ReflectComponent,
};

/// The data structure containing the minimum tilemap data needed for each tile as well as manages
/// chunk access and setup
///
/// Each tile should only contain the bare minimum data needed for you to figure out what it is. Any
/// data that is not the same for every single tile of that type should be stored as a component
/// on that tiles entity which is managed through the [`Chunk`]
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct Tilemap {
    /// Struct containing [`Entity`] mappings to the [`Chunk`](super::chunk::Chunk)s that hold tile data
    chunks: Chunks,
}

impl Tilemap {
    /// Creates a new [`Tilemap`] using the given vector of vectors of the given T.
    ///
    ///
    pub fn new_builder<T>(
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

        for y in 0..y_chunk_amount {
            let mut x_vec: Vec<Entity> = vec![];
            for x in 0..x_chunk_amount {
                let entity = commands
                    .spawn(Chunk::<T>::new_default(
                        ChunkPos::new(x, y),
                        50,
                        50,
                        UVec2::new(max_chunk_size.x, max_chunk_size.y),
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
            .spawn(Tilemap::new(chunks))
            .push_children(flattened_chunk_entities.as_slice())
            .id();
        tilemap_entity
    }

    /// Creates a new [`Tilemap`] out of the given chunks struct
    pub fn new(chunks: Chunks) -> Tilemap {
        Self { chunks }
    }

    /// Gets the chunk entity that has the tile_info for the given TilePos
    pub fn get_chunk_for_tile_pos(&mut self, tile_pos: TilePos) -> Option<Entity> {
        self.chunks.get_chunk(tile_pos)
    }

    /// Returns the max size that a chunk can be
    pub fn get_chunks_max_size(&mut self) -> UVec2 {
        self.chunks.max_chunk_size()
    }
}

#[cfg(test)]
mod tests {
    use crate::map::tilemap::Tilemap;
}
