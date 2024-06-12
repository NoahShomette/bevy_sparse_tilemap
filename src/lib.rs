//! # Bevy_Sparse_Tilemap
//!
//! A relatively simple, sparse style tilemap for the Bevy Game Engine.
//!
//! ## Tilemap Construction
//!
//! bevy_sparse_tilemap includes a [`TilemapBuilder`](tilemap_builder::TilemapBuilder) SystemParam that is used to spawn and setup a tilemap
//! correctly in the world.
//!
//! ```rust
//! # use bevy::prelude::{Commands, Entity, Reflect, UVec2};
//! # use bevy_sparse_tilemap::map::chunk::ChunkSettings;
//! # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
//! # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
//! # use bst_map_layer_derive::MapLayer;
//! #
//! # #[derive(MapLayer, Clone, Copy, Default)]
//! # pub enum MapLayers {
//! #     #[default]
//! #     Main,
//! #     Secondary,
//! # }
//! #
//! # #[derive(Default, Copy, Clone, Reflect)]
//! # struct TileData(u8, u8);
//! #
//! #
//! fn spawn_tilemap(mut tilemap_builder: TilemapBuilder<TileData, MapLayers>, mut commands: Commands) {
//!     tilemap_builder.new_tilemap_with_main_layer(
//!         TilemapLayer::new_dense_default(5000,5000),
//!         ChunkSettings {
//!             max_chunk_size: UVec2::new(100, 100),
//!         },
//!     );
//!     let tilemap = tilemap_builder.spawn_tilemap();
//! }
//!
//! ```
//! ## Tilemap Access
//!
//! bevy_sparse_tilemap includes a handy [`TilemapManager`](tilemap_manager::TilemapManager) system
//! param that has a bevy of helper functions to make accessing, editing, and interacting with tilemaps
//! that much easier.
//!
//! ```rust
//! # use bevy::prelude::{Commands, Entity, Reflect, UVec2};
//! # use bevy_sparse_tilemap::map::chunk::ChunkSettings;
//! # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
//! # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
//! # use bevy_sparse_tilemap::tilemap_manager::TilemapManager;
//! # use bevy_sparse_tilemap::TilePos;
//! # use bst_map_layer_derive::MapLayer;
//! #
//! # #[derive(MapLayer, Clone, Copy, Default)]
//! # pub enum MapLayers {
//! #     #[default]
//! #     Main,
//! #     Secondary,
//! # }
//! #
//! # #[derive(Default, Copy, Clone, Reflect)]
//! # struct TileData(u8, u8);
//! #
//!
//! fn access(tilemap_manager: TilemapManager<TileData, MapLayers>, mut commands: Commands) {
//!     let tile_data = tilemap_manager.get_tile_data(TilePos::new(9,16)).unwrap();
//!
//!     //    
//!
//! }
//!
//! ```

pub mod cell;
pub mod integrations;
pub mod map;
pub mod square;
pub mod tilemap_builder;
pub mod tilemap_manager;

use bevy::app::App;
use bevy::prelude::Plugin;

pub use bst_map_layer_derive::MapLayer;

use crate::integrations::IntegrationsPlugin;

/// Plugin provided to setup **BevySparseTilemap**
///
/// Mostly only required when enabling optional features and integrations
pub struct SparseTilemapPlugin;

impl Plugin for SparseTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IntegrationsPlugin);
    }
}
