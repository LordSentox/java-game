//! # The courier adventurer
//!
//! # Special ability
//! The courier can transfer a transferable card to any player, regardless of
//! their respective positions.

use super::{Adventurer, AdventurerInfo};
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Courier {
    pos: FieldPos
}

impl Courier {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Courier {}

impl AdventurerInfo for Courier {
    /// The Courier can trade with anyone, no matter the position, so this
    /// function always returns true.
    fn can_trade_with(&self, _other: &dyn Positionable) -> bool { true }
}
