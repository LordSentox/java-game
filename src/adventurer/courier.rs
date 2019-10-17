//! # The courier adventurer
//!
//! # Special ability
//! The courier can transfer a transferable card to any player, regardless of
//! their respective positions.

use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Courier {
    pos: FieldPos
}

impl Courier {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}
