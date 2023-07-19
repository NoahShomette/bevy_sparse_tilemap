A Tilemap crate for the Bevy game engine with a focus on logic and size.

## Features
- Sparse-set style Tilemaps
  - Only the minimum amount of data is stored for each tile. Furthermore tiles that don't need it don't get their own Entity
  - Built in API to handle spawning, despawning, and accessing tiles optional entities
- Massive tilemap sizes.
  - Because of the above feature, maps can be ludicrously large and have very good performance (Up to and above 13,000x13,000).
- Purely focused on the tilemap logic - rendering is left to the user

## What about bevy_ecs_tilemap?

`bevy_ecs_tilemap` is a fabulous crate that will probably cover most of your needs in an easier to use plugin.

You should use `bevy_ecs_tilemap` if:
- You don't need very large maps (sub 500x500 in my experience)
- You want every tile to be its own Entity (This crate tries to avoid unnecessary entities and uses a Voxel like approach)
- You want a more mature and more feature rich plugin
- You want tilemap rendering handled for you

You should use `bevy_sparse_tilemap` if:
- You want very very large maps (bevy_sparse_tilemap can easily reach map sizes of 13,000x13,000)
- You don't mine a more cumbersome api in pursuit of the first goal
- You are willing to implement your own tilemap rendering (This crate does have an optional feature for integration with `bevy_fast_tilemap` however it is not the focus of this crate)
