use bevy::prelude::Resource;

pub mod tilemap_manager;
pub mod errors;


#[derive(Resource, Default)]
struct LayerIndex<MapLayer>(MapLayer);

