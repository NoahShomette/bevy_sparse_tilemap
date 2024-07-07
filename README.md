# `bevy_sparse_tilemap`

[![Crates.io](https://img.shields.io/crates/v/bevy_sparse_tilemap)](https://crates.io/crates/bevy_sparse_tilemap)
[![docs](https://docs.rs/bevy_sparse_tilemap/badge.svg)](https://docs.rs/bevy_sparse_tilemap/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/NoahShomette/bevy_sparse_tilemap/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/bevy_sparse_tilemap)](https://crates.io/crates/bevy_sparse_tilemap)

A Tilemap crate for the Bevy game engine with a focus on large map sizes and ECS sparse maps

## Features

### `ECS Sparse Tilemaps`

> `bevy_sparse_tilemap` only spawns the minimum amount of entities neccesary and doesn't one-to-one map tiles to entities. This allows it to dodge current Bevy performance issues related to rendering and unlocks a substantially higher baseline performance for extra large maps.
>
> Tiles in `bevy_sparse_tilemap` are stored on each chunk using custom storage. `bevy_sparse_tilemap` does support spawning entities mapped to specific tiles at will for when you really need to save extra information or to gain access to the ECS when needed for specific tile types.
>
> `bevy_sparse_tilemap` has a built in API to handle spawning, despawning, and accessing tiles optional entities as well as tile data.
>
> Internally this crate supports both Sparse and Dense internal storage. Both are still ECS sparse (minimal amount of entities spawned) but sparse maps don't require the user to supply data for every tile.

### `Multiple Map Types`

> `bevy_sparse_tilemap` supports different map types using a generics trait based system that provides great flexibility to support many different map types.
>
> Currently supported:
>
> - Hexagon
> - Square

### `Massive Map Sizes`

> Because of the ECS Sparse nature of the crate, `bevy_sparse_tilemap` dodges current limitations in Bevy related to rendering performance issues. These limitations are what other tilemap crates run into and inhibits their performance. This allows `bevy_sparse_tilemap` to have exceptional performance as a baseline, allowing maps as large as 15k x 15k easily with smart rendering using something like [bevy_fast_tilemap](https://github.com/Droggelbecher/bevy-fast-tilemap).

### `Tilemap Logic Only`

> This crate focuses purely on the tilemap logic and leaves the rendering to the user.

## Examples

See [Docs.rs](https://docs.rs/bevy_sparse_tilemap/latest/bevy_sparse_tilemap/) for documentation on how to use `bevy_sparse_tilemap` as well as brief examples.

See [Github Examples](https://github.com/NoahShomette/bevy_sparse_tilemap/tree/main/examples) for longer examples of each feature of the crate.

## What about `bevy_ecs_tilemap`?

`bevy_ecs_tilemap` is a fabulous crate that will probably cover most of your needs in an easier to use plugin and you
should reach for that first in most situations.

You should use `bevy_ecs_tilemap` if:

- You don't need very large maps (`bevy_ecs_tilemap` runs into performance issues around 200x200 in my testing)
- You want every tile to be its own Entity for ECS integration (This crate tries to avoid unnecessary entities and uses a Voxel like approach)
- You want a more mature and more feature rich plugin
- You want tilemap rendering handled for you

You should use `bevy_sparse_tilemap` if:

- You want very very large maps, `bevy_sparse_tilemap` can reach substantially larger map sizes compared to `bevy_ecs_tilemap`. (The bevy_fast_tilemap_example currently spawns a 15000x15000 tile map and runs at around 900 fps)
- You are willing to implement your own tilemap rendering (This crate has an example for integration with `bevy_fast_tilemap` however that is not currently a feature that is natively supported by this crate)

## Bevy Version

| BST Version | Bevy Version |
| :---------: | :----------: |
|     0.3     |     0.14     |
|     0.2     |     0.13     |
|     0.1     |     0.13     |
