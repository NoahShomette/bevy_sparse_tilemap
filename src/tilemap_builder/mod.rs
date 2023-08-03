pub mod tilemap_layer_builder;

use crate::map::chunk::{ChunkSettings, Chunks};
use crate::map::tilemap::Tilemap;
use crate::tilemap_builder::tilemap_layer_builder::{break_layer_into_chunks, TilemapLayer};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{BuildChildren, Commands, Entity, Local, Resource, UVec2};
use bevy::utils::HashMap;
use std::marker::PhantomData;

/// Info to build a Tilemap with
struct TilemapBuilderInfo<TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::map::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    main_layer: TilemapLayer<TileData>,
    layer_info: HashMap<u32, TilemapLayer<TileData>>,
    chunk_settings: ChunkSettings,
    map_size: UVec2,
    // All phantom data below
    tm_phantom: PhantomData<TilemapMarker>,
    td_phantom: PhantomData<TileData>,
    ml_phantom: PhantomData<MapLayer>,
}

impl<TilemapMarker, TileData, MapLayer> Default
    for TilemapBuilderInfo<TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::map::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    fn default() -> Self {
        Self {
            main_layer: Default::default(),
            layer_info: Default::default(),
            chunk_settings: ChunkSettings {
                max_chunk_size: UVec2::new(50, 50),
            },
            map_size: Default::default(),
            tm_phantom: PhantomData::default(),
            td_phantom: PhantomData::default(),
            ml_phantom: PhantomData::default(),
        }
    }
}

/// A private resource for use in [`TilemapBuilder`].
///
/// Holds the data and settings for the newly spawned tilemap
#[derive(Resource)]
struct TilemapBuilderInfoInstance<TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::map::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    tilemap_builder_info: TilemapBuilderInfo<TilemapMarker, TileData, MapLayer>,
    tm_phantom: PhantomData<TilemapMarker>,
    td_phantom: PhantomData<TileData>,
    ml_phantom: PhantomData<MapLayer>,
}

impl<TilemapMarker, TileData, MapLayer> Default
    for TilemapBuilderInfoInstance<TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::map::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    fn default() -> Self {
        Self {
            tilemap_builder_info: TilemapBuilderInfo::default(),
            tm_phantom: PhantomData::default(),
            td_phantom: PhantomData::default(),
            ml_phantom: PhantomData::default(),
        }
    }
}

#[derive(SystemParam)]
pub struct TilemapBuilder<'w, 's, TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::map::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    tm_phantom: PhantomData<TilemapMarker>,
    td_phantom: PhantomData<TileData>,
    ml_phantom: PhantomData<MapLayer>,
    tilemap_info: Local<'s, TilemapBuilderInfoInstance<TilemapMarker, TileData, MapLayer>>,
    commands: Commands<'w, 's>,
}

impl<'w, 's, TilemapMarker, TileData, MapLayer>
    TilemapBuilder<'w, 's, TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::map::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    /// Function for internal use to take the settings from TilemapBuilder and spawn the actual
    /// tilemap returning the TilemapEntity
    pub fn spawn_tilemap(&mut self) -> Entity {
        let mut chunks = break_layer_into_chunks(
            &self.tilemap_info.tilemap_builder_info.main_layer,
            self.tilemap_info
                .tilemap_builder_info
                .chunk_settings
                .max_chunk_size,
        );

        let mut chunk_entities: Vec<Vec<Entity>> = vec![];

        let map_x = chunks[0].len();

        for y in 0..chunks.len() {
            let mut vec: Vec<Entity> = vec![];
            for x in 0..map_x {
                let entity = self.commands.spawn(chunks[y].remove(0)).id();
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
            self.tilemap_info
                .tilemap_builder_info
                .chunk_settings
                .max_chunk_size,
        );

        let tilemap_entity = self
            .commands
            .spawn(Tilemap::<TilemapMarker>::new(chunks))
            .push_children(flattened_chunk_entities.as_slice())
            .id();
        tilemap_entity
    }

    pub fn new_tilemap_with_main_layer(
        &mut self,
        layer_data: TilemapLayer<TileData>,
        chunk_settings: ChunkSettings,
    ) {
        self.tilemap_info.tilemap_builder_info = TilemapBuilderInfo {
            main_layer: layer_data,
            layer_info: Default::default(),
            chunk_settings,
            map_size: Default::default(),
            tm_phantom: Default::default(),
            td_phantom: Default::default(),
            ml_phantom: Default::default(),
        }
    }

    pub fn add_layer(&mut self, layer_data: TilemapLayer<TileData>, map_layer: MapLayer) {
        assert_eq!(
            self.tilemap_info.tilemap_builder_info.map_size,
            layer_data.dimensions(),
            "New layers must be the same size as the map dimensions"
        );
        self.tilemap_info
            .tilemap_builder_info
            .layer_info
            .insert(map_layer.to_bits(), layer_data);
    }
}
