//! # The pilot adventurer
//!
//! # Special ability
//! Once a turn the pilot may use their special ability to fly to any island
//! fild they like. Unlike with a helicopter card, they may not take anybody
//! with them.

use super::{Adventurer, AdventurerInfo};
use crate::map::Full as FullMap;
use crate::math::Vec2;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Pilot {
    pos: Vec2<u8>
}

impl Pilot {
    pub fn implicit_special() -> bool { false }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Pilot {}

impl AdventurerInfo for Pilot {
    /// The pilot may fly anywhere on the map where they can stand when they use
    /// their special ability.
    fn special_moves(&self, _map: &FullMap) -> Vec<Vec2<u8>> { unimplemented!() }
}
