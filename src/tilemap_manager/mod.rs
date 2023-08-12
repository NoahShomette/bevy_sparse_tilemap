use bevy::prelude::{Entity, FromWorld, Resource};

mod errors;
mod tilemap_manager;

pub use errors::TilemapManagerError;
pub use tilemap_manager::TilemapManager;

#[derive(Resource, Default)]
pub(crate) struct LayerIndex<MapLayer>(pub(crate) MapLayer);

#[derive(Resource)]
pub(crate) struct MapEntity(pub(crate) Option<Entity>);

impl Default for MapEntity {
    fn default() -> Self {
        Self(None)
    }
}
