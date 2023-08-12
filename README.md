# `bevy_sparse_tilemap`
[![Crates.io](https://img.shields.io/crates/v/bevy_sparse_tilemap)](https://crates.io/crates/bevy_sparse_tilemap)
[![docs](https://docs.rs/bevy_sparse_tilemap/badge.svg)](https://docs.rs/bevy_sparse_tilemap/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/StarArawn/bevy_sparse_tilemap/blob/main/LICENSE)
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
 fn spawn_tilemap(mut tilemap_builder: TilemapBuilder<TileData, MapLayers>, mut commands: Commands) {
     tilemap_builder.new_tilemap_with_main_layer(
         TilemapLayer::new_dense_default(5000,5000),
         ChunkSettings {
             max_chunk_size: UVec2::new(100, 100),
         },
     );
     let tilemap = tilemap_builder.spawn_tilemap();
 }
 ```
 ## Tilemap Access

 bevy_sparse_tilemap includes a handy `TilemapManager` system
 param that has a bevy of helper functions to make accessing, editing, and interacting with tilemaps
 that much easier.

 ```rust
 fn access(tilemap_manager: TilemapManager<TileData, MapLayers>, mut commands: Commands) {
     let tile_data = tilemap_manager.get_tile_data(TilePos::new(9,16)).unwrap();

     //    

 }
 ```

## What about `bevy_ecs_tilemap`?

`bevy_ecs_tilemap` is a fabulous crate that will probably cover most of your needs in an easier to use plugin and you 
should reach for that first.

You should use `bevy_ecs_tilemap` if:
- You don't need very large maps (sub 200x200 in my experience)
- You want every tile to be its own Entity for ECS integration (This crate tries to avoid unnecessary entities and uses a Voxel like approach)
- You want a more mature and more feature rich plugin
- You want tilemap rendering handled for you

You should use `bevy_sparse_tilemap` if:
- You want very very large maps (bevy_sparse_tilemap can reach substantially larger map sizes)
- Basically the above is the main reason, giant maps that are more cumbersome to work with but can be millions of tiles
- You don't mind a more cumbersome api in pursuit of the first goal
- You are willing to implement your own tilemap rendering (This crate does have an optional feature for integration with `bevy_fast_tilemap` however it is not the focus of this crate and does not do a lot)
