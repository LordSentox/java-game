use crate::math::Vec2;

// TODO: A proc macro would be nicer, but I wanted to spare us an extra crate
// TODO  just for that. In case there will be more procedural macros, we should
// TODO  create such a crate though.
#[macro_export]
macro_rules! impl_pos {
    ($s:ident, $p:ident) => {
        impl Positionable for $s {
            fn pos(&self) -> { self.$p }
            fn set_pos(&self, pos: Vec2<u8>) -> { self.pos = pos; }
        }
    };
}

pub trait Positionable {
    fn pos(&self) -> Vec2<u8>;

    fn set_pos(&mut self, pos: Vec2<u8>);
}
