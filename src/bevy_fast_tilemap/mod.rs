//! Integration with Bevy_fast_tilemap
//!
//! Todo:
//! - Add bevy_fast
//!

use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_fast_tilemap::FastTileMapPlugin;

pub struct BevyFastTilemapFeaturePlugin;

impl Plugin for BevyFastTilemapFeaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FastTileMapPlugin);
    }
}
