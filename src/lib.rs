#[doc = include_str!("../README.md")]
pub mod integrations;
pub mod map;
pub mod square;
pub mod tilemap_builder;
pub mod tilemap_manager;

use bevy::app::App;
use bevy::prelude::Plugin;

pub use bst_map_layer_derive::MapLayer;
pub use lettuces::*;

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
