pub mod tilemap_layer_builder;

use crate::map::chunk::{Chunk, ChunkSettings, Chunks, LayerType, MapChunkLayer};
use crate::map::{MapData, MapLayer, Tilemap};
use crate::tilemap_builder::tilemap_layer_builder::TilemapLayer;
use bevy::prelude::{BuildChildren, Commands, Entity, UVec2};
use bevy::utils::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Information to construct a Tilemap
pub struct TilemapBuilder<TileData, MapLayers, MapChunk, MapType>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Clone + Copy + Send + Sync + 'static,
    MapChunk: MapChunkLayer<TileData> + Send + Sync + 'static + Default,
    MapType: MapData + Default,
{
    main_layer: Option<TilemapLayer<TileData>>,
    layer_info: HashMap<u32, TilemapLayer<TileData>>,
    chunk_settings: ChunkSettings,
    map_size: UVec2,
    map_type: MapType,
    chunk_conversion_settings: MapChunk::ConversionSettings,
    // All phantom data below
    td_phantom: PhantomData<TileData>,
    ml_phantom: PhantomData<MapLayers>,
    ct_phantom: PhantomData<MapChunk>,
}

impl<TileData, MapLayers, MapChunk, MapType> Default
    for TilemapBuilder<TileData, MapLayers, MapChunk, MapType>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Clone + Copy + Send + Sync + 'static,
    MapChunk: MapChunkLayer<TileData> + Send + Sync + 'static + Default,

    MapType: MapData + Default,
{
    fn default() -> Self {
        Self {
            main_layer: None,
            layer_info: Default::default(),
            chunk_settings: ChunkSettings {
                max_chunk_size: UVec2::new(50, 50),
            },
            map_size: Default::default(),
            map_type: Default::default(),
            td_phantom: PhantomData::default(),
            ml_phantom: PhantomData::default(),
            ct_phantom: PhantomData::default(),
            chunk_conversion_settings: MapChunk::ConversionSettings::default(),
        }
    }
}

impl<TileData, MapLayers, MapChunk, MapType>
    TilemapBuilder<TileData, MapLayers, MapChunk, MapType>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Clone + Copy + Send + Sync + 'static,
    MapChunk: MapChunkLayer<TileData> + Send + Sync + 'static + Default,
    MapType: MapData + Default,
{
    /// Converts all the data from the [`SystemParam`] and spawns the
    /// tilemap returning the Tilemaps [`Entity`]
    #[must_use]
    pub fn spawn_tilemap(mut self, commands: &mut Commands) -> Option<Entity> {
        let Some(layer) = self.main_layer.take() else {
            return None;
        };

        let mut chunks = self.create_new_chunks_from_layer(
            &layer,
            self.chunk_conversion_settings,
            self.chunk_settings.max_chunk_size,
        );

        let layers: Vec<(u32, TilemapLayer<TileData>)> = self.layer_info.drain().collect();

        for (id, layer) in layers {
            self.add_layer_to_chunks(id, &mut chunks, &layer, self.chunk_settings.max_chunk_size)
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
        Some(tilemap_entity)
    }

    /// Makes a new [`TilemapBuilder`] with the given [`TilemapLayer`] as the main layer.
    pub fn new(
        layer_data: TilemapLayer<TileData>,
        map_type: MapType,
        chunk_settings: ChunkSettings,
        chunk_conversion_settings: MapChunk::ConversionSettings,
    ) -> Self {
        let dimensions = layer_data.dimensions();
        TilemapBuilder::<TileData, MapLayers, MapChunk, MapType> {
            main_layer: Some(layer_data),
            layer_info: Default::default(),
            chunk_settings,
            map_size: dimensions,
            map_type,
            td_phantom: Default::default(),
            ml_phantom: Default::default(),
            ct_phantom: PhantomData::default(),
            chunk_conversion_settings,
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

    /// Function which creates new chunks and inserts the given tilemap layer into those chunks
    pub fn create_new_chunks_from_layer(
        &mut self,
        tilemap_layer: &TilemapLayer<TileData>,
        chunk_conversion_settings: MapChunk::ConversionSettings,
        max_chunk_size: UVec2,
    ) -> Vec<Vec<Chunk<MapChunk, TileData>>>
    where
        TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    {
        return match tilemap_layer {
            TilemapLayer::Sparse(data, map_size, entities) => {
                let mut chunks = self.map_type.break_hashmap_into_chunks(
                    MapLayers::default(),
                    data,
                    map_size.clone(),
                    max_chunk_size,
                    chunk_conversion_settings,
                );
                self.map_type.add_entities_to_layer(
                    MapLayers::default().to_bits(),
                    &mut chunks,
                    entities,
                );
                chunks
            }
            TilemapLayer::Dense(data, entities) => {
                let mut chunks = self.map_type.break_data_vecs_into_chunks(
                    data,
                    max_chunk_size,
                    chunk_conversion_settings,
                );
                self.map_type.add_entities_to_layer(
                    MapLayers::default().to_bits(),
                    &mut chunks,
                    entities,
                );
                chunks
            }
        };
    }

    /// Adds the given layer to the tilemap
    pub fn add_layer_to_chunks(
        &mut self,
        map_layer: u32,
        chunks: &mut Vec<Vec<Chunk<MapChunk, TileData>>>,
        tilemap_layer: &TilemapLayer<TileData>,
        max_chunk_size: UVec2,
    ) {
        match tilemap_layer {
            TilemapLayer::Sparse(data, .., entities) => {
                for y in chunks.iter_mut() {
                    for chunk in y.iter_mut() {
                        chunk.add_layer(map_layer, LayerType::Sparse(HashMap::new()));
                    }
                }
                for (cell, tile_data) in data.iter() {
                    let chunk_pos =
                        MapType::into_chunk_pos(*cell, self.map_type.conversion_settings());
                    let chunk = &mut chunks[chunk_pos.y() as usize][chunk_pos.x() as usize];
                    chunk.set_tile_data(
                        map_layer,
                        MapChunk::into_chunk_cell(*cell, &chunk.cell_conversion_settings),
                        tile_data.clone(),
                    );
                }
                self.map_type
                    .add_entities_to_layer(map_layer, chunks, entities);
            }
            TilemapLayer::Dense(data, entities) => {
                for y in chunks.iter_mut() {
                    for chunk in y.iter_mut() {
                        let vec = self.map_type.break_data_vecs_down_into_chunk_data(
                            &data,
                            chunk.chunk_pos,
                            max_chunk_size,
                        );
                        chunk.add_layer(map_layer, LayerType::Dense(vec));
                    }
                }
                self.map_type
                    .add_entities_to_layer(map_layer, chunks, entities);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as bevy_sparse_tilemap;

    use crate::map::chunk::ChunkSettings;
    use crate::tilemap_builder::tilemap_layer_builder::TilemapLayer;
    use crate::tilemap_builder::TilemapBuilder;
    use crate::tilemap_manager::TilemapManager;
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
