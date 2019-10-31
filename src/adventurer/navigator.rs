//! # The navigator adventurer
//!
//! # Special ability
//! The navigator can move other players in the primary directions (up, down,
//! left, right). They can move two players each one tile or less or one player
//! two tiles or less per action point unless they themselves use a movement
//! action in between. The moved players may not use any special movement
//! ability while they are being navigated.

use super::{Adventurer, AdventurerInfo};
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Navigator {
    pos: FieldPos,
    extra_push: bool
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            pos: FieldPos::new(),
            extra_push: false
        }
    }

    pub fn implicit_special() -> bool { false }

    pub fn can_move_others() -> bool { true }
}

impl Adventurer for Navigator {
    /// The navigator may move another adventurer when they have an action point
    /// left, or if there has been a push immediately before which already
    /// cost an action point.
    fn can_move_other(&self, act_points: u8) -> bool { act_points != 0 || self.extra_push }

    /// When someone is moved, either an action point is spent or there is no
    /// extra move anymore.
    fn on_move_other(&mut self, act_points: &mut u8) {
        if self.extra_push {
            self.extra_push = false;
        }
        else {
            *act_points -= 1;
            self.extra_push = true;
        }
    }

    /// If any extra movement of another adventurer was possible, it is reset
    /// when the navigator moves.
    fn on_move(&mut self) { self.extra_push = false; }
}

impl AdventurerInfo for Navigator {}
