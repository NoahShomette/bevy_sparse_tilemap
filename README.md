# `bevy_sparse_tilemap`

[![Crates.io](https://img.shields.io/crates/v/bevy_sparse_tilemap)](https://crates.io/crates/bevy_sparse_tilemap)
[![docs](https://docs.rs/bevy_sparse_tilemap/badge.svg)](https://docs.rs/bevy_sparse_tilemap/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/NoahShomette/bevy_sparse_tilemap/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/bevy_sparse_tilemap)](https://crates.io/crates/bevy_sparse_tilemap)

A Tilemap crate for the Bevy game engine with a focus on large map sizes and ECS sparse maps

## Features

- Sparse-set style tilemaps
  - Only the minimum amount of data is stored for each tile. Furthermore tiles that don't need it don't get their own Entity
  - Built in API to handle spawning, despawning, and accessing tiles optional entities
- Massive tilemap sizes.
  - Because of the above feature, maps can be ludicrously large and have very good performance.
- Purely focused on the tilemap logic - rendering is left to the user

## Tilemap Construction

bevy_sparse_tilemap includes a `TilemapBuilder` SystemParam that is used to spawn and setup a tilemap
correctly in the world.

```rust

 # use bevy::prelude::{Commands, Entity, Reflect, UVec2};
 # use bevy_sparse_tilemap::map::chunk::ChunkSettings;
 # use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
 # use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
 # use bst_map_layer_derive::MapLayer;
 #    use bevy_sparse_tilemap::square::{
 # map_chunk_layer::SquareChunkLayer, map_data::SquareMapData, map_data::SquareMapDataConversionSettings, map_chunk_layer::SquareChunkLayerConversionSettings
 # };
 #
 # #[derive(MapLayer, Clone, Copy, Default)]
 # pub enum MapLayers {
 #     #[default]
 #     Main,
 #     Secondary,
 # }
 #
 # #[derive(Default, Copy, Clone, Reflect, Hash)]
 # struct TileData(u8, u8);
 #
 #

 fn spawn_tilemap(mut commands: Commands) {
     let chunk_conversion_settings = SquareChunkLayerConversionSettings {
         max_chunk_dimensions: UVec2 { x: 100, y: 100 },
     };

    let map_conversion_settings = SquareMapDataConversionSettings {
        max_chunk_dimensions: UVec2 { x: 100, y: 100 },
    };

     let mut tilemap_builder = TilemapBuilder::<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData,
         >::new(
         TilemapLayer::new_dense_default(10000, 10000),
         SquareMapData {
             conversion_settings: SquareMapDataConversionSettings {
                 max_chunk_dimensions: UVec2::new(100, 100),
                 },
             },
         ChunkSettings {
             max_chunk_size: UVec2::new(100, 100),
         },
         chunk_conversion_settings,
         map_conversion_settings,
     );

     let Some(tilemap) = tilemap_builder.spawn_tilemap(&mut commands)
         else {
             return;
     };
 }
```

## Tilemap Access

bevy_sparse_tilemap includes a handy `TilemapManager` system
param that has a bevy of helper functions to make accessing, editing, and interacting with tilemaps
that much easier.

```rust
# use bevy::prelude::{Commands, Entity, Reflect, UVec2};
# use bevy_sparse_tilemap::map::chunk::ChunkSettings;
# use bevy_sparse_tilemap::tilemap_builder::tilemap_layer_builder::TilemapLayer;
# use bevy_sparse_tilemap::tilemap_builder::TilemapBuilder;
# use bevy_sparse_tilemap::tilemap_manager::TilemapManager;
# use lettuces::cell::Cell;
# use bst_map_layer_derive::MapLayer;
# use bevy_sparse_tilemap::square::{
    map_data::SquareMapData,
 map_chunk_layer::SquareChunkLayer,
 };
#
# #[derive(MapLayer, Clone, Copy, Default)]
# pub enum MapLayers {
#     #[default]
#     Main,
#     Secondary,
# }
#
# #[derive(Default, Copy, Clone, Reflect, Hash)]
# struct TileData(u8, u8);
#

 fn access(tilemap_manager: TilemapManager<TileData, MapLayers, SquareChunkLayer<TileData>, SquareMapData>, mut commands: Commands) {
     let tile_data = tilemap_manager.get_tile_data(Cell::new(9,16)).unwrap();

     // do something with the tilemap access here

 }
```

## What about `bevy_ecs_tilemap`?

`bevy_ecs_tilemap` is a fabulous crate that will probably cover most of your needs in an easier to use plugin and you
should reach for that first.

You should use `bevy_ecs_tilemap` if:

- You don't need very large maps (sub 200x200 in my testing)
- You want every tile to be its own Entity for ECS integration (This crate tries to avoid unnecessary entities and uses a Voxel like approach)
- You want a more mature and more feature rich plugin
- You want tilemap rendering handled for you

You should use `bevy_sparse_tilemap` if:

- You want very very large maps (bevy_sparse_tilemap can reach substantially larger map sizes. The bevy_fast_tilemap_example currently spawns a 15000x15000 tile map)
- Basically the above is the main reason, giant maps that are (currently) more cumbersome to work with but can be millions of tiles
- You want tile shapes other than square (other shapes are planned but not implemented and not on the radar currently)
- You are willing to implement your own tilemap rendering (This crate has an example for integration with `bevy_fast_tilemap` however that is not currently a feature of this crate)

## Bevy Version

| BST Version | Bevy Version |
| :---------: | :----------: |
|     0.3     |     0.14     |
|     0.2     |     0.13     |
|     0.1     |     0.13     |
