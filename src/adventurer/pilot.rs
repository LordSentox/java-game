//! # The pilot adventurer
//!
//! # Special ability
//! Once a turn the pilot may use their special ability to fly to any island
//! fild they like. Unlike with a helicopter card, they may not take anybody
//! with them.

use crate::math::Vec2;
use crate::positionable::Positionable;

use super::{Adventurer, AdventurerInfo};

#[derive(Positionable)]
pub struct Pilot {
    pos: Vec2<u8>
}

impl Pilot {
    pub fn implicit_special() -> bool { false }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Pilot {}

impl AdventurerInfo for Pilot {}
