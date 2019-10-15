pub trait Positionable {
    fn pos(&self) -> Vec2<u8>;

    fn set_pos(&mut self, pos: Vec2<u8>);
}
