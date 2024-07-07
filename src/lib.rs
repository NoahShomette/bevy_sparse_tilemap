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
//! # Usage Docs
//!
//! Brief overview of core concepts of `bevy_sparse_tilemap`
//!
//! ## [`Cell`](crate::lettuces::cell::Cell), [`ChunkPos`](crate::map::chunk::ChunkPos), [`ChunkCell`](crate::map::chunk::ChunkCell)
//!
//! For the below example imagine we had a map that was 150 x 150 tiles and had chunks of 25 tiles each. This means the map is made up of 36 chunks each containing 625 tiles.
//!
//! `bevy_sparse_tilemap` uses three separate "tile positions". For most usage, you should just treat the map as one big thing and use `Cell`, ignoring the fact it's chunked.
//!
//! - `Cell`: Represents a logical position on the map from the maps (0, 0) to the (map_size_x, map_size_y). Eg (x-120, y-57).
//! - `ChunkPos`: Represents the position of a chunk in the map. For a Cell in (0,0) -> (24,24) the chunk that "owns" those tiles would be at (0,0). For a cell (x-120, y-57) the ChunkPos would be (4,2).
//! - `ChunkCell`: Represents a position on the tilemap inside of a chunk. For a cell (x-120, y-57) the ChunkPos would be (4,2) and the ChunkCell would be (20, 7).
//!
//! ## Tilemap Construction
//!
//! Create new tilemaps by using the [`TilemapBuilder`](crate::tilemap_builder::TilemapBuilder). This helper manages splitting data across chunks, spawning chunks, map, adding new layers, and more related helpers.
//!
//! You should *always* use the tilemap builder unless for some reason you absolutely cannot (please open an issue if you run into this).
//!
//! ```
//! # use bevy::prelude::{Commands, Entity, Reflect, UVec2};
//! # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
//! # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
//! # use bst_map_layer_derive::MapLayer;
//! #    use bevy_sparse_tilemap::square::{
//! # map_chunk_layer::SquareChunkLayer, map_data::SquareMapData, map_chunk_layer::SquareChunkSettings
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
//!
//!     let mut tilemap_builder = TilemapBuilder::<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData,
//!         >::new(
//!         TilemapLayer::new_dense_default(10000, 10000),
//!         SquareMapData {
//!             max_chunk_size: UVec2::new(100, 100)
//!             },
//!         SquareChunkSettings {
//!             max_chunk_size: UVec2 { x: 100, y: 100 },
//!             }
//!     );
//!
//!     let Some(tilemap) = tilemap_builder.spawn_tilemap(&mut commands)
//!         else {
//!             return;
//!     };
//! }
//!
//! ```
//! ## Tilemap Access
//!
//! `bevy_sparse_tilemap` includes a handy `[TilemapManager`](crate::tilemap_manager::TilemapManager) system param that has a bevy of helper functions to make accessing, editing, and interacting with tilemaps that much easier. You don't technically need to use this to interact with the maps however it makes it substantially easier as the guts of this crate can be a bit convuluted.
//!
//! ```
//! # use bevy::prelude::{Commands, Entity, Reflect, UVec2, Resource, Res};
//! # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
//! # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
//! # use bevy_sparse_tilemap::tilemap_manager::TilemapManager;
//! # use lettuces::cell::Cell;
//! # use bst_map_layer_derive::MapLayer;
//! # use bevy_sparse_tilemap::square::{
//! #    map_data::SquareMapData,
//! # map_chunk_layer::SquareChunkLayer,
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
//! # #[derive(Resource)]
//! # pub struct MapEntity(Entity);
//!
//!  fn access(mut tilemap_manager: TilemapManager<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData>, mut commands: Commands, map_entity: Res<MapEntity>) {
//!     // We have to set the TilemapManager to the desired tilemap
//!     tilemap_manager.set_tilemap_entity(map_entity.0);
//!     // And set the manager to whatever layer we want to affect. Defaults to the default layer of the enum
//!     tilemap_manager.set_layer(MapLayers::Main);
//!     let tile_data = tilemap_manager.get_tile_data(Cell::new(9,16)).unwrap();
//!
//!     // do something with the tilemap access here
//!
//!  }
//!
//! ```
//!

/// Implements a hexagonal map type. See the [Hexagon Example](https://github.com/NoahShomette/bevy_sparse_tilemap/blob/main/examples/hexagon.rs) for an overview of how to use it
#[cfg(feature = "hex")]
pub mod hex;
pub mod map;
/// Implements a square map type. See the [Square Example](https://github.com/NoahShomette/bevy_sparse_tilemap/blob/main/examples/square.rs) for an overview of how to use it
#[cfg(feature = "square")]
pub mod square;
/// A helper used to construct new tilemaps. See [`TilemapBuilder`](crate::tilemap_builder::TilemapBuilder) for more details
pub mod tilemap_builder;
/// A system param used to interact with tilemaps. See [`TilemapManager`](crate::tilemap_manager::TilemapManager) for more details
pub mod tilemap_manager;

pub use bst_map_layer_derive::MapLayer;
/// Re-export [Lettuces](https://crates.io/crates/lettuces)
pub mod lettuces {
    pub use lettuces::*;
}
