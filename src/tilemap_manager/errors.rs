use bevy::ecs::query::QueryEntityError;

#[derive(thiserror::Error, Debug)]
pub enum TilemapManagerError {
    #[error("A Chunk does not exist for the given ChunkPos")]
    InvalidChunkPos,
    #[error("A Chunk entity does not exist for the given ChunkPos")]
    ChunkEntityDoesNotExist(#[from] QueryEntityError),
    #[error("An Entity does not exist for the given TilePos")]
    TileEntityDoesNotExist,
    #[error("TileData does not exist for the given TilePos")]
    TileDataDoesNotExist,
}
