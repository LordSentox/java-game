//! # The navigator adventurer
//!
//! # Special ability
//! The navigator can move other players in the primary directions (up, down,
//! left, right). They can move two players each one tile or less or one player
//! two tiles or less per action point unless they themselves use a movement
//! action in between. The moved players may not use any special movement
//! ability while they are being navigated.

use crate::math::Vec2;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Navigator {
    pos: Vec2<u8>
}

impl Navigator {
    pub fn implicit_special() -> bool { false }

    pub fn can_move_others() -> bool { true }
}

impl Adventurer for Navigator {
    fn can_move_other(&self, _act_points: u8) -> bool { unimplemented!() }

    fn on_move_other(&mut self, _act_points: &mut u8) { unimplemented!() }
}

impl AdventurerInfo for Navigator {}
