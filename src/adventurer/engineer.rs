//! # The engineer adventurer
//!
//! # Special ability
//! The engineer can shore up two tiles for one action point, unless they
//! perform a movement action in between those two drains.

use crate::positionable::Positionable;

use super::{Adventurer, AdventurerInfo};

#[derive(Positionable)]
pub struct Engineer {
    pos: FieldPos
}

impl Engineer {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Engineer {
    /// If the engineer moves, they may not drain a second tile without spending
    /// an extra action point.
    fn on_move(&mut self, _act_points: &mut u8) { unimplemented!() }

    /// Instead of always costing an action point. If for the last drain action
    /// an action point was already consumed, the action points remain
    /// unchanged.
    fn on_drain(&mut self, _act_points: &mut u8) { unimplemented!() }
}

impl AdventurerInfo for Engineer {}
