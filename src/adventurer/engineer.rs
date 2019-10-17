//! # The engineer adventurer
//!
//! # Special ability
//! The engineer can shore up two tiles for one action point, unless they
//! perform a movement action in between those two drains.

use crate::math::Vec2;
use crate::positionable::Positionable;

use super::{Adventurer, AdventurerInfo};

#[derive(Positionable)]
pub struct Engineer {
    pos: Vec2<u8>
}

impl Engineer {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Engineer {
    fn on_move(&mut self, _act_points: &mut u8) { unimplemented!() }

    fn on_drain(&mut self, _act_points: &mut u8) { unimplemented!() }
}

impl AdventurerInfo for Engineer {}
