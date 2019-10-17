//! # The diver adventurer
//!
//! # Special ability
//! The diver can swim through flooded or gone tiles, but only if there is or
//! was an island tile before. They can swim in all primary directions (Up,
//! Down, Left, Right) until they land on the island tile they want to end or
//! the first non-flooded non-gone tile.

use super::{Adventurer, AdventurerInfo};
use crate::map::Full as FullMap;
use crate::math::Vec2;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Diver {
    pos: Vec2<u8>
}

impl Diver {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Diver {}

impl AdventurerInfo for Diver {
    /// The diving move set on the map. Returns all positions the Diver can dive
    /// to and stand on after the action.
    fn special_moves(&self, _map: &FullMap) -> Vec<Vec2<u8>> { unimplemented!() }
}
