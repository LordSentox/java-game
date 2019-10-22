//! Utilities to generate maps

use super::{BlackWhite, Full};
use crate::math::Rect;

/// Generate a valid black and white island.
///
/// # Parameters
/// `limit_rect` - The bounds in which the map must be generated
///
/// # Panics
/// If `limit_rect` cannot hold the 24 tiles necessary for a valid map.
pub fn black_white_island(_limit_rect: Rect<u8>) -> BlackWhite { unimplemented!() }

/// Generate a random distribution of the 24 possible tiles on a black and white
/// map.
///
/// # Parameters
/// `shape` - The shape the island should have.
///
/// # Panics
/// If the shape is not a valid black and white map
pub fn filled_island(_shape: &BlackWhite) -> Full { unimplemented!() }
