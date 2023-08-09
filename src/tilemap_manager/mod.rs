use bevy::prelude::Resource;

mod errors;
mod tilemap_manager;

pub use errors::TilemapManagerError;
pub use tilemap_manager::TilemapManager;

#[derive(Resource, Default)]
pub(crate) struct LayerIndex<MapLayer>(pub(crate) MapLayer);
