//! Core Tilemap concept and

pub mod chunk;
pub mod tilemap;


/// A layer used for identifying and accessing multiple layers of a [`Tilemap`]
///
/// This trait can be derived for enums with `#[derive(MapLayer)]`.
pub trait MapLayer: Sized {
    const DEFAULT: u32 = 1u32 << 0;
    /// Converts the layer to a bitmask.
    fn to_bits(&self) -> u32;
    /// Creates a layer bitmask with all bits set to 1.
    fn all_bits() -> u32;
}

impl<L: MapLayer> MapLayer for &L {
    fn to_bits(&self) -> u32 {
        L::to_bits(self)
    }

    fn all_bits() -> u32 {
        L::all_bits()
    }
}


