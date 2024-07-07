#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::unwrap_used
)]
#![doc = include_str!("../README.md")]
//! ## Tilemap Construction
//!
//! ```
//! # use bevy::prelude::{Commands, Entity, Reflect, UVec2};
//! # use bevy_sparse_tilemap::map::chunk::ChunkSettings;
//! # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
//! # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
//! # use bst_map_layer_derive::MapLayer;
//! #    use bevy_sparse_tilemap::square::{
//! # map_chunk_layer::SquareChunkLayer, map_data::SquareMapData, map_data::SquareMapDataConversionSettings, map_chunk_layer::SquareChunkLayerConversionSettings
//! # };
//! #
//! # #[derive(MapLayer, Clone, Copy, Default)]
//! # pub enum MapLayers {
//! #     #[default]
//! #     Main,
//! #     Secondary,
//! # }
//! #
//! # #[derive(Default, Copy, Clone, Reflect, Hash)]
//! # struct TileData(u8, u8);
//! #
//! #
//!
//! fn spawn_tilemap(mut commands: Commands) {
//!     let chunk_conversion_settings = SquareChunkSettings {
//!         max_chunk_size: UVec2 { x: 100, y: 100 },
//!     };
//!
//!     let mut tilemap_builder = TilemapBuilder::<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData,
//!         >::new(
//!         TilemapLayer::new_dense_default(10000, 10000),
//!         SquareMapData {
//!             conversion_settings: SquareMapDataConversionSettings {
//!                 max_chunk_dimensions: UVec2::new(100, 100),
//!                 },
//!             },
//!         ChunkSettings {
//!             max_chunk_size: UVec2::new(100, 100),
//!         },
//!         chunk_conversion_settings,
//!     );
//!
//!     let Some(tilemap) = tilemap_builder.spawn_tilemap(&mut commands)
//!         else {
//!             return;
//!     };
//! }
//! ```
//! ## Tilemap Access
//! bevy_sparse_tilemap includes a handy `TilemapManager` systemp aram that has a bevy of helper functions to make accessing, editing, and interacting with tilemaps that much easier.
//! ```
//! # use bevy::prelude::{Commands, Entity, Reflect, UVec2, Resource, Res};
//! # use bevy_sparse_tilemap::map::chunk::ChunkSettings;
//! # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
//! # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
//! # use bevy_sparse_tilemap::tilemap_manager::TilemapManager;
//! # use lettuces::cell::Cell;
//! # use bst_map_layer_derive::MapLayer;
//! # use bevy_sparse_tilemap::square::{
//!     map_data::SquareMapData,
//!  map_chunk_layer::SquareChunkLayer,
//!  };
//! #
//! # #[derive(MapLayer, Clone, Copy, Default)]
//! # pub enum MapLayers {
//! #     #[default]
//! #     Main,
//! #     Secondary,
//! # }
//! #
//! # #[derive(Default, Copy, Clone, Reflect, Hash)]
//! # struct TileData(u8, u8);
//! # #[derive(Resource)]
//! # pub struct MapEntity(Entity);
//!
//!  fn access(tilemap_manager: TilemapManager<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData>, mut commands: Commands, map_entity: Res<MapEntity>) {
//!     // We have to set the TilemapManager to the desired tilemap
//!     tilemap_manager.set_tilemap_entity(map_entity.0);
//!     // And set the manager to whatever layer we want to affect. Defaults to the default layer of the enum
//!     tilemap_manager.set_layer(MapLayers::Main);
//!     let tile_data = tilemap_manager.get_tile_data(Cell::new(9,16)).unwrap();
//!
//!     // do something with the tilemap access here
//!
//!  }
//! ```
//!

/// Implements a hexagonal map type. See the [Hexagon Example](https://github.com/NoahShomette/bevy_sparse_tilemap/blob/main/examples/hexagon.rs) for an overview of how to use it
#[cfg(feature = "hex")]
pub mod hex;
pub mod map;
/// Implements a square map type. See the [Hexagon Example](https://github.com/NoahShomette/bevy_sparse_tilemap/blob/main/examples/square.rs) for an overview of how to use it
#[cfg(feature = "square")]
pub mod square;
/// A helper used to construct new tilemaps. See [`TilemapBuilder] for more details
pub mod tilemap_builder;
/// A system param used to interact with tilemaps. See [`TilemapManager`] for more details
pub mod tilemap_manager;

pub use bst_map_layer_derive::MapLayer;
pub use lettuces::*;
