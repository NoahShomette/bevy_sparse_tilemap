pub mod tilemap_layer_builder;

use crate::map::chunk::{ChunkSettings, Chunks};
use crate::map::{MapLayer, Tilemap};
use crate::tilemap_builder::tilemap_layer_builder::{
    add_layer_to_chunks, break_layer_into_chunks, TilemapLayer,
};
use bevy::prelude::{BuildChildren, Commands, Entity, UVec2};
use bevy::utils::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Information to construct a Tilemap
pub struct TilemapBuilder<TileData, MapLayers>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Clone + Copy + Send + Sync + 'static,
{
    main_layer: TilemapLayer<TileData>,
    layer_info: HashMap<u32, TilemapLayer<TileData>>,
    chunk_settings: ChunkSettings,
    map_size: UVec2,
    // All phantom data below
    td_phantom: PhantomData<TileData>,
    ml_phantom: PhantomData<MapLayers>,
}

impl<TileData, MapLayers> Default for TilemapBuilder<TileData, MapLayers>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Clone + Copy + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            main_layer: Default::default(),
            layer_info: Default::default(),
            chunk_settings: ChunkSettings {
                max_chunk_size: UVec2::new(50, 50),
            },
            map_size: Default::default(),
            td_phantom: PhantomData::default(),
            ml_phantom: PhantomData::default(),
        }
    }
}

impl<TileData, MapLayers> TilemapBuilder<TileData, MapLayers>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Clone + Copy + Send + Sync + 'static,
{
    /// Converts all the data from the [`SystemParam`] and spawns the
    /// tilemap returning the Tilemaps [`Entity`]
    #[must_use]
    pub fn spawn_tilemap(self, commands: &mut Commands) -> Entity {
        let mut chunks =
            break_layer_into_chunks(&self.main_layer, self.chunk_settings.max_chunk_size);

        for (id, layer) in self.layer_info.iter() {
            add_layer_to_chunks(*id, &mut chunks, layer, self.chunk_settings.max_chunk_size)
        }

        let mut chunk_entities: Vec<Vec<Entity>> = vec![];

        let map_x = chunks[0].len();

        for y in 0..chunks.len() {
            let mut vec: Vec<Entity> = vec![];
            for _ in 0..map_x {
                let entity = commands.spawn(chunks[y].remove(0)).id();
                vec.push(entity);
            }
            chunk_entities.push(vec);
        }

        let mut flattened_chunk_entities: Vec<Entity> = vec![];

        for chunk_entity in chunk_entities.iter_mut() {
            flattened_chunk_entities.extend(chunk_entity.iter().cloned())
        }

        let chunks = Chunks::new(
            Chunks::new_chunk_entity_grid(chunk_entities),
            self.chunk_settings.max_chunk_size,
        );

        let tilemap_entity = commands
            .spawn(Tilemap::new(chunks))
            .push_children(flattened_chunk_entities.as_slice())
            .id();
        tilemap_entity
    }

    /// Sets the [`TilemapBuilder`] to the default with the addition of the given [`TilemapLayer`] as
    /// the main layer.
    pub fn new_tilemap_with_main_layer(
        layer_data: TilemapLayer<TileData>,
        chunk_settings: ChunkSettings,
    ) -> Self {
        let dimensions = layer_data.dimensions();
        TilemapBuilder::<TileData, MapLayers> {
            main_layer: layer_data,
            layer_info: Default::default(),
            chunk_settings,
            map_size: dimensions,
            td_phantom: Default::default(),
            ml_phantom: Default::default(),
        }
    }

    /// Adds the given [`TilemapLayer`] to the tilemap keyed to the given [`MapLayers`]
    pub fn add_layer(&mut self, layer_data: TilemapLayer<TileData>, map_layer: MapLayers) {
        assert_eq!(
            self.map_size,
            layer_data.dimensions(),
            "New layers must be the same size as the map dimensions"
        );
        self.layer_info.insert(map_layer.to_bits(), layer_data);
    }
}

#[cfg(test)]
mod tests {
    use crate as bevy_sparse_tilemap;

    use crate::map::chunk::ChunkSettings;
    use crate::tilemap_builder::tilemap_layer_builder::TilemapLayer;
    use crate::tilemap_builder::TilemapBuilder;
    use crate::tilemap_manager::TilemapManager;
    use crate::TilePos;
    use bevy::ecs::system::SystemState;
    use bevy::math::UVec2;
    use bevy::prelude::World;
    use bevy::utils::hashbrown::HashMap;
    use bst_map_layer_derive::MapLayer;

    #[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
    struct TileData(u8);

    #[derive(MapLayer, Default, Debug, PartialEq, Eq, Clone, Copy)]
    enum MapLayers {
        #[default]
        Main,
        Secondary,
    }

    pub struct MainMap;
}
