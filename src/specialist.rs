use crate::map::Full as FullMap;
use crate::math::Vec2;

/// Trait implemented by adventurers, which have their own abilities the
/// character representing them must know. The trait already contains the
/// default implementations that any adventurer can perform.
pub trait Specialist {
    /// List of the moves the adventurer can perform from the provided position,
    /// when they use their special ability.
    fn special_moves(from: Vec2<u8>, map: &FullMap) -> Vec<Vec2<u8>> { Vec::new() }

    /// Called when the character has moved. If something special should happen
    /// to the adventurer afterwards, this must be implemented.
    fn on_move(&mut self) {}

    /// Is set true, when the adventurer can move other adventurer's characters.
    fn can_move_others() -> bool { false }

    /// Called, after the adventurer has moved another player. Since only the
    /// navigator can move others, this will only be implemented for them.
    fn on_move_other(&mut self) {}

    /// List of the special positions the adventurer can drain, other than the
    /// usual direct neighbours (up, down, left, right). If the adventurer
    /// makes a special implementation of this function, these will not be
    /// returned, only the new positions.
    fn special_drains(from: Vec2<u8>) -> Vec<Vec2<u8>> { Vec::new() }

    /// Called, after the adventurer has drained a tile. For most of the
    /// adventurers, nothing will happen, but for instance for the engineer
    /// this will handle the extra drain.
    fn on_drain(&mut self) {}

    /// Determines, if the adventurer can transfer from their current position
    /// to a character at the other position. By default, they can only
    /// transfer to characters on the same position as them.
    fn can_transfer(pos: Vec2<u8>, other_pos: Vec2<u8>) { pos == other_pos }
}
