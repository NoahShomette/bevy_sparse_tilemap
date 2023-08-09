use bevy::app::{App, Plugin};

#[cfg(feature = "bevy_fast_tilemap")]
pub use crate::integrations::bevy_fast_tilemap::BevyFastTilemapFeaturePlugin;
#[cfg(feature = "bevy_fast_tilemap")]
pub mod bevy_fast_tilemap;

pub struct IntegrationsPlugin;

impl Plugin for IntegrationsPlugin{
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_fast_tilemap")]
        app.add_plugins(BevyFastTilemapFeaturePlugin);
    }
}