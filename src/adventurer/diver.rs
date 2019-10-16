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
