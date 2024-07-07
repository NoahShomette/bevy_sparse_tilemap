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

## Examples

See [Docs.rs](https://docs.rs/bevy_sparse_tilemap/latest/bevy_sparse_tilemap/) for documentation on how to use `bevy_sparse_tilemap` as well as brief examples.

See [Github Examples](https://github.com/NoahShomette/bevy_sparse_tilemap/tree/main/examples) for longer examples of each feature of the crate.

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
