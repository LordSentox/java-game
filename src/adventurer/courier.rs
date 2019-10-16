use crate::math::Vec2;

impl_pos!(Courier, pos);
pub struct Courier {
    pos: Vec2<u8>
}

impl Courier {
    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}
