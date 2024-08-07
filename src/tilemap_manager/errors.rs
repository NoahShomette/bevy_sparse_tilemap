﻿use bevy::ecs::query::QueryEntityError;

/// Errors returned by a [`super::TilemapManager`]
#[derive(thiserror::Error, Debug)]
pub enum TilemapManagerError {
    /// A chunk does not exist for the given [`ChunkPos`](crate::map::chunk::ChunkPos)
    #[error("A Chunk does not exist for the given ChunkPos")]
    InvalidChunkPos,

    /// A chunk does not exist for the given [`ChunkPos`](crate::map::chunk::ChunkPos)
    #[error("A Chunk entity does not exist for the given ChunkPos")]
    ChunkEntityDoesNotExist(#[from] QueryEntityError),

    /// A tile entity does not exist for the given [`ChunkCell`](crate::map::chunk::ChunkCell)
    #[error("An Entity does not exist for the given ChunkCell")]
    TileEntityDoesNotExist,

    /// `TileData` does not exist for the given [`ChunkCell`](crate::map::chunk::ChunkCell)
    #[error("TileData does not exist for the given ChunkCell")]
    TileDataDoesNotExist,
}
