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
