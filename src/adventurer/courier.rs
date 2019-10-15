use crate::math::Vec2;

pub struct Courier;

impl Courier {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}
