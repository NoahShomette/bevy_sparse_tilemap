use crate::map::chunk::{Chunk, ChunkPos};
use crate::map::{MapLayer, Tilemap};
use crate::tilemap_manager::TilemapManagerError;
use crate::tilemap_manager::{LayerIndex, MapEntity};
use bevy::ecs::system::SystemParam;
use bevy::math::UVec2;
use bevy::prelude::{Children, Commands, DespawnRecursiveExt, Entity, Local, Query};
use std::hash::Hash;
use std::ops::Deref;

use crate::TilePos;

/// A [`SystemParam`] used to access and interact with a [`Tilemap`]
///
/// # IMPORTANT
///
/// You **MUST** set the [TilemapManager] to a specific tilemap using [`set_tilemap_entity()`](TilemapManager::set_tilemap_entity) before you use the Tilemap Manager.
/// If you don't these functions will panic.
///
/// # Internal [`SystemParam`]s
/// - Query<(Entity, &mut Tilemap, Option<&'static Children>)>
/// - Query<(Entity, &mut Chunk<TileData>, Option<&'static Children>)>,
#[derive(SystemParam)]
pub struct TilemapManager<'w, 's, TileData, MapLayers>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Default + Clone + Copy + Send + Sync + 'static,
{
    tilemap_query: Query<'w, 's, (Entity, &'static mut Tilemap, Option<&'static Children>)>,
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
    layer_index: Local<'s, LayerIndex<MapLayers>>,
    map_entity: Local<'s, MapEntity>,
}

impl<'w, 's, TileData, MapLayers> TilemapManager<'w, 's, TileData, MapLayers>
where
    TileData: Hash + Clone + Copy + Sized + Default + Send + Sync + 'static,
    MapLayers: MapLayer + Default + Clone + Copy + Send + Sync + 'static,
{
    /// Returns the [`Tilemap`] entity that this tilemap manager is set to affect
    pub fn tilemap_entity(&self) -> Option<Entity> {
        self.map_entity.deref().0
    }

    /// Sets the [`Tilemap`] entity that this tilemap manager is set to affect. This must be set before
    /// using any other functions that modify the tilemap
    pub fn set_tilemap_entity(&mut self, entity: Entity) {
        *self.map_entity = MapEntity(Some(entity));
    }

    /// Returns the currently set [`MapLayers`]
    pub fn layer(&self) -> MapLayers {
        self.layer_index.0
    }

    /// Sets the [`MapLayers`] that all future operations will be conducted upon.
    ///
    /// # Note
    ///
    /// The selected layer will persist across system runs
    pub fn on_layer(&mut self, map_layer: MapLayers) {
        *self.layer_index = LayerIndex(map_layer)
    }

    /// Returns the [`Tilemap`]s dimensions.
    pub fn dimensions(&self) -> Result<UVec2, TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;

        let chunks = tilemap.chunks().chunk_counts();
        let average_chunk_size = self
            .chunk_query
            .get(
                tilemap
                    .get_chunk(ChunkPos::new(0, 0))
                    .ok_or(TilemapManagerError::InvalidChunkPos)?,
            )?
            .1
            .get_chunk_dimensions();

        let max_x_chunk = self
            .chunk_query
            .get(
                tilemap
                    .get_chunk(ChunkPos::new(chunks.x - 1, 0))
                    .ok_or(TilemapManagerError::InvalidChunkPos)?,
            )?
            .1
            .get_chunk_dimensions();

        let max_y_chunk = self
            .chunk_query
            .get(
                tilemap
                    .get_chunk(ChunkPos::new(0, chunks.y - 1))
                    .ok_or(TilemapManagerError::InvalidChunkPos)?,
            )?
            .1
            .get_chunk_dimensions();

        Ok(UVec2::new(
            (average_chunk_size.x * (chunks.x - 1)) + max_x_chunk.x,
            (average_chunk_size.y * (chunks.y - 1)) + max_y_chunk.y,
        ))
    }

    /// Gets the tile data for the given [`TilePos`] if it exists.
    pub fn get_tile_data(&self, tile_pos: TilePos) -> Result<TileData, TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, chunk, _) = self.chunk_query.get(
            tilemap
                .get_chunk_for_tile_pos(tile_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;
        chunk
            .get_tile_data(
                self.layer_index.0,
                tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
            )
            .ok_or(TilemapManagerError::TileDataDoesNotExist)
    }

    /// Sets the tile data for the given [`TilePos`] if it exists.
    pub fn sets_tile_data(
        &mut self,
        tile_data: TileData,
        tile_pos: TilePos,
    ) -> Result<(), TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, mut chunk, _) = self.chunk_query.get_mut(
            tilemap
                .get_chunk_for_tile_pos(tile_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;
        Ok(chunk.set_tile_data(
            self.layer_index.0,
            tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
            tile_data,
        ))
    }

    /// Gets the [`Entity`] for the given [`TilePos`] if it exists.
    pub fn get_tile_entity(&self, tile_pos: TilePos) -> Result<Entity, TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, chunk, _) = self.chunk_query.get(
            tilemap
                .get_chunk_for_tile_pos(tile_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;
        chunk
            .get_tile_entity(
                self.layer_index.0,
                tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
            )
            .ok_or(TilemapManagerError::TileEntityDoesNotExist)
    }

    /// Sets the [`Entity`] for the given [`TilePos`]. Prefer to use [`get_or_spawn_tile_entity`](TilemapManager::get_or_spawn_tile_entity).
    pub fn set_tile_entity(
        &mut self,
        tile_pos: TilePos,
        entity: Entity,
    ) -> Result<(), TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, mut chunk, _) = self.chunk_query.get_mut(
            tilemap
                .get_chunk_for_tile_pos(tile_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;

        chunk.set_tile_entity(
            self.layer_index.0,
            tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
            entity,
        );

        Ok(())
    }

    /// Gets the [`Entity`] for the given [`TilePos`] if it exists or spawns one and returns that if it
    /// doesn't.
    pub fn get_or_spawn_tile_entity(
        &mut self,
        tile_pos: TilePos,
    ) -> Result<Entity, TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, mut chunk, _) = self.chunk_query.get_mut(
            tilemap
                .get_chunk_for_tile_pos(tile_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;

        let entity = chunk
            .get_tile_entity(
                self.layer_index.0,
                tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
            )
            .unwrap_or_else(|| {
                let entity = self.commands.spawn_empty().id();
                chunk.set_tile_entity(
                    self.layer_index.0,
                    tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
                    entity,
                );
                entity
            });

        Ok(entity)
    }

    /// Gets the [`Entity`] for the given [`TilePos`] if it exists or spawns one and returns that if it
    /// doesn't.
    pub fn despawn_tile_entity(&mut self, tile_pos: TilePos) -> Result<(), TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, chunk, _) = self.chunk_query.get(
            tilemap
                .get_chunk_for_tile_pos(tile_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;

        if let Some(entity) = chunk.get_tile_entity(
            self.layer_index.0,
            tile_pos.into_chunk_tile_pos(tilemap.get_chunks_max_size()),
        ) {
            self.commands.entity(entity).despawn_recursive();
        };

        Ok(())
    }

    /// Returns the [`Chunk`] data for the given [`ChunkPos`] if it exists
    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Result<&Chunk<TileData>, TilemapManagerError> {
        let (_, tilemap, _) = self.tilemap_query.get(
            self.map_entity
                .deref()
                .0
                .expect("TilemapManager must have a tilemap entity set"),
        )?;
        let (_, chunk, _) = self.chunk_query.get(
            tilemap
                .get_chunk(chunk_pos)
                .ok_or(TilemapManagerError::InvalidChunkPos)?,
        )?;
        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use crate as bevy_sparse_tilemap;

    use crate::map::chunk::ChunkSettings;
    use crate::tilemap_builder::tilemap_layer_builder::TilemapLayer;
    use crate::tilemap_builder::TilemapBuilder;
    use crate::tilemap_manager::tilemap_manager::TilemapManager;
    use crate::TilePos;
    use bevy::ecs::system::{Commands, SystemState};
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

    #[test]
    fn tilemap_manager_dense_access() {
        let mut world = World::new();

        let mut system_state: SystemState<(Commands, TilemapManager<(i32, i32), MapLayers>)> =
            SystemState::new(&mut world);
        let (mut commands, mut tilemap_manager) = system_state.get_mut(&mut world);
        assert_eq!(tilemap_manager.layer(), MapLayers::Main);
        tilemap_manager.on_layer(MapLayers::Secondary);
        assert_eq!(tilemap_manager.layer(), MapLayers::Secondary);
        tilemap_manager.on_layer(MapLayers::Main);

        #[rustfmt::skip]
            let vecs = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0),(4, 0), (5, 0), (6, 0), (7, 0)],
            vec![(0, 1), (1, 1), (2, 1), (3, 1),(4, 1), (5, 1), (6, 1), (7, 1)],
            vec![(0, 2), (1, 2), (2, 2), (3, 2),(4, 2), (5, 2), (6, 2), (7, 2)],
            vec![(0, 3), (1, 3), (2, 3), (3, 3),(4, 3), (5, 3), (6, 3), (7, 3)],
            vec![(0, 4), (1, 4), (2, 4), (3, 4),(4, 4), (5, 4), (6, 4), (7, 4)],
            vec![(0, 5), (1, 5), (2, 5), (3, 5),(4, 5), (5, 5), (6, 5), (7, 5)],
            vec![(0, 6), (1, 6), (2, 6), (3, 6),(4, 6), (5, 6), (6, 6), (7, 6)],
            vec![(0, 7), (1, 7), (2, 7), (3, 7),(4, 7), (5, 7), (6, 7), (7, 7)],
            vec![(0, 8), (1, 8), (2, 8), (3, 8),(4, 8), (5, 8), (6, 8), (7, 8)]
        ];
        let tilemap_builder = TilemapBuilder::<(i32, i32), MapLayers>::new_tilemap_with_main_layer(
            TilemapLayer::new_dense_from_vecs(vecs),
            ChunkSettings {
                max_chunk_size: UVec2::new(5, 5),
            },
        );
        let map_entity = tilemap_builder.spawn_tilemap(&mut commands);

        system_state.apply(&mut world);

        let (_, mut tilemap_manager) = system_state.get_mut(&mut world);
        tilemap_manager.set_tilemap_entity(map_entity);
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(0, 0)).unwrap(),
            (0, 0)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(7, 8)).unwrap(),
            (7, 8)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(7, 0)).unwrap(),
            (7, 0)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(0, 6)).unwrap(),
            (0, 6)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(2, 1)).unwrap(),
            (2, 1)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(4, 4)).unwrap(),
            (4, 4)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(7, 8)).unwrap(),
            (7, 8)
        );
        // Testing bounds
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(7, 9)).is_err(),
            true
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(8, 7)).is_err(),
            true
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(0, 9)).is_err(),
            true
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(8, 0)).is_err(),
            true
        );
    }

    #[test]
    fn tilemap_manager_sparse_access() {
        let mut world = World::new();

        let mut system_state: SystemState<(Commands, TilemapManager<(i32, i32), MapLayers>)> =
            SystemState::new(&mut world);
        let (mut commands, mut tilemap_manager) = system_state.get_mut(&mut world);
        assert_eq!(tilemap_manager.layer(), MapLayers::Main);
        tilemap_manager.on_layer(MapLayers::Secondary);
        assert_eq!(tilemap_manager.layer(), MapLayers::Secondary);
        tilemap_manager.on_layer(MapLayers::Main);

        let mut hashmap: HashMap<TilePos, (i32, i32)> = HashMap::new();
        hashmap.insert(TilePos::new(0, 0), (0, 0));
        hashmap.insert(TilePos::new(31, 31), (31, 31));

        let tilemap_builder = TilemapBuilder::<(i32, i32), MapLayers>::new_tilemap_with_main_layer(
            TilemapLayer::new_sparse_from_hashmap(32, 32, hashmap),
            ChunkSettings {
                max_chunk_size: UVec2::new(5, 5),
            },
        );
        let map_entity = tilemap_builder.spawn_tilemap(&mut commands);

        system_state.apply(&mut world);

        let (_, mut tilemap_manager) = system_state.get_mut(&mut world);
        tilemap_manager.set_tilemap_entity(map_entity);

        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(0, 0)).unwrap(),
            (0, 0)
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(31, 31)).unwrap(),
            (31, 31)
        );
        // Testing bounds
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(7, 9)).is_err(),
            true
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(8, 7)).is_err(),
            true
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(0, 9)).is_err(),
            true
        );
        assert_eq!(
            tilemap_manager.get_tile_data(TilePos::new(8, 0)).is_err(),
            true
        );
    }

    #[test]
    fn tilemap_manager_dimensions() {
        let mut world = World::new();

        let mut system_state: SystemState<(Commands, TilemapManager<(i32, i32), MapLayers>)> =
            SystemState::new(&mut world);
        let (mut commands, _) = system_state.get_mut(&mut world);

        #[rustfmt::skip]
            let vecs = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
            vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1)],
            vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2)],
            vec![(0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3)],
            vec![(0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4)],
            vec![(0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5)],
            vec![(0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6)],
            vec![(0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7)],
            vec![(0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8)]
        ];
        let tilemap_builder = TilemapBuilder::<(i32, i32), MapLayers>::new_tilemap_with_main_layer(
            TilemapLayer::new_dense_from_vecs(vecs),
            ChunkSettings {
                max_chunk_size: UVec2::new(5, 5),
            },
        );
        let map_entity = tilemap_builder.spawn_tilemap(&mut commands);
        system_state.apply(&mut world);
        let (_, mut tilemap_manager) = system_state.get_mut(&mut world);
        tilemap_manager.set_tilemap_entity(map_entity);
        tilemap_manager.on_layer(MapLayers::Main);

        assert_eq!(tilemap_manager.dimensions().unwrap(), UVec2::new(8, 9));
    }
}
