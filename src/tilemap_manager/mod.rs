use bevy::prelude::Resource;

mod tilemap_manager;


#[derive(Resource, Default)]
struct LayerIndex<MapLayer>(MapLayer);

