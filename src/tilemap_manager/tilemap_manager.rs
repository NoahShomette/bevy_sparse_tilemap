use crate::tilemap_manager::LayerIndex;
use crate::{Chunk, MapLayer, TilePos, Tilemap};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Children, Commands, Entity, Local, Query, Resource, UVec2};

#[derive(SystemParam)]
pub struct TilemapManager<'w, 's, TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    tilemap_query: Query<
        'w,
        's,
        (
            Entity,
            &'static mut Tilemap<TilemapMarker>,
            Option<&'static Children>,
        ),
    >,
    chunk_query: Query<
        'w,
        's,
        (
            Entity,
            &'static mut Chunk<TileData>,
            Option<&'static Children>,
        ),
    >,
    commands: Commands<'w, 's>,
    layer_index: Local<'s, LayerIndex<MapLayer>>,
}

impl<'w, 's, TilemapMarker, TileData, MapLayer>
    TilemapManager<'w, 's, TilemapMarker, TileData, MapLayer>
where
    TilemapMarker: Send + Sync + 'static,
    TileData: Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayer: crate::MapLayer + Clone + Copy + Send + Sync + Default + 'static,
{
    pub fn spawn_map(
        &mut self,
        tilemap_tile_data: Vec<Vec<TileData>>,
        max_chunk_size: UVec2,
    ) -> Entity {
        Tilemap::<TilemapMarker>::spawn_tilemap(
            tilemap_tile_data,
            max_chunk_size,
            &mut self.commands,
        )
    }

    pub fn layer(&self) -> &MapLayer {
        &self.layer_index.0
    }

    pub fn on_layer(&mut self, map_layer: MapLayer) {
        *self.layer_index = LayerIndex(map_layer)
    }

    pub fn get_tile_data(&self, tile_pos: TilePos) -> TileData {
        let (_, tilemap, _) = self.tilemap_query.single();
        let (_, chunk, _) = self
            .chunk_query
            .get(
                tilemap
                    .get_chunk_for_tile_pos(tile_pos.into_chunk_pos(tilemap.get_chunks_max_size()))
                    .unwrap(),
            )
            .unwrap();
        chunk
            .get_tile_data(
                self.layer_index.0,
                tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
            )
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::tilemap_manager::tilemap_manager::TilemapManager;
    use crate::{Chunk, ChunkPos, ChunkTilePos, MapLayer, TilePos};
    use bevy::ecs::system::SystemState;
    use bevy::math::UVec2;
    use bevy::prelude::World;
    use bevy::utils::hashbrown::HashMap;

    #[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
    struct TileData(u8);

    #[derive(MapLayer, Default, Debug, PartialEq, Eq, Clone, Copy)]
    enum MapLayers {
        #[default]
        Main,
        Secondary,
    }

    pub struct MainMap;

    #[test]
    fn test_tilemap_manager() {
        let mut world = World::new();

        let mut system_state: SystemState<TilemapManager<MainMap, (i32, i32), MapLayers>> =
            SystemState::new(&mut world);
        let mut tilemap_manager = system_state.get_mut(&mut world);
        assert_eq!(tilemap_manager.layer(), &MapLayers::Main);
        tilemap_manager.on_layer(MapLayers::Secondary);
        assert_eq!(tilemap_manager.layer(), &MapLayers::Secondary);

        
        let vecs = vec![
            vec![(0, 0), (1, 2), (2, 0), (3, 0),(0, 0), (1, 2), (2, 0), (3, 34)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8),(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4),(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(0, 0), (1, 2), (2, 0), (3, 0),(0, 0), (1, 2), (2, 0), (67, 054)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8),(58, 587), (1, 2), (2, 0), (3, 0)],
            vec![(8, 4), (9, 6), (10, 1), (11, 4),(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(669, 64), (1, 2), (2, 0), (3, 0),(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(4, 1), (5, 6), (6, 7), (7, 8),(0, 0), (1, 2), (2, 0), (3, 0)],
            vec![(8, 4), (58, 6), (10, 1), (11, 4),(0, 0), (1, 2), (57, 01), (57, 56)]
        ];
        tilemap_manager.spawn_map(vecs, UVec2::new(5, 5));
        system_state.apply(&mut world);

        let mut tilemap_manager = system_state.get_mut(&mut world);
        tilemap_manager.on_layer(MapLayers::Main);
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(7, 8)), (57, 56));
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(7, 0)), (3, 34));
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(0, 0)), (0, 0));
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(0, 6)), (669, 64));
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(2, 1)), (6, 7));
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(4, 4)), (58, 587));
        assert_eq!(tilemap_manager.get_tile_data(TilePos::new(7, 9)), (67, 054));

        /*
        let mut hashmap: HashMap<ChunkTilePos, (u32, u32)> = HashMap::new();
        hashmap.insert(ChunkTilePos::new(0, 0), (50, 60));
        let mut chunk = Chunk::new_uniform(ChunkPos::new(0, 0), 30, 30, (0u32, 0u32));
        chunk.add_sparse_layer(MapLayers::Secondary, Some(hashmap));
        assert_eq!(
            chunk
                .get_tile_data(MapLayers::Secondary, ChunkTilePos { x: 0, y: 0 })
                .unwrap(),
            (50, 60)
        );

         */
    }
}
