//! # The explorer adventurer
//!
//! # Special ability
//! The explorer can move diagonally

use super::{Adventurer, AdventurerInfo};
use crate::map::Full as FullMap;
use crate::math::Vec2;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Explorer {
    pos: Vec2<u8>
}

impl Explorer {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Explorer {}

impl AdventurerInfo for Explorer {
    fn special_moves(&self, _map: &FullMap) -> Vec<Vec2<u8>> { unimplemented!() }
}
