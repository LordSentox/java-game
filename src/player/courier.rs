use crate::math::Vec2;
use crate::player::CardTransferer;

pub struct Courier {
    name: String,
    pos:  Vec2<u8>
}

impl CardTransferer for Courier {
    fn can_transfer(_: (u8, u8), _: (u8, u8)) -> bool { true }
}
