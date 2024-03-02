use bevy::prelude::{Entity, Resource};

mod errors;
mod tilemap_manager;

pub use errors::TilemapManagerError;
pub use tilemap_manager::TilemapManager;

/// A local resource for the tilemap manager that holds the currently selected map layer
#[derive(Resource, Default)]
pub(crate) struct LayerIndex<MapLayer>(pub(crate) MapLayer);

/// A local resource for the tilemap that holds the map entity that the tilemap manager is working with
#[derive(Resource)]
pub(crate) struct MapEntity(pub(crate) Option<Entity>);

impl Default for MapEntity {
    fn default() -> Self {
        Self(None)
    }
}
