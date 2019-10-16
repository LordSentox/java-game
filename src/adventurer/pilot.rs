use crate::math::Vec2;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Pilot {
    pos: Vec2<u8>
}

impl Pilot {
    pub fn implicit_special() -> bool { false }

    pub fn can_move_others() -> bool { false }
}
