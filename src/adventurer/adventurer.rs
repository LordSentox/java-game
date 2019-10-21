//! Traits that describe the actions an Adventurer can take that may be unique
//! to them.

use crate::map::{Full as FullMap, MapExt};
use crate::math::Vec2;
use crate::positionable::Positionable;

/// The specific actions an adventurer can take, also taking the current state
/// of the turn into account. The standard implementation is usually just the
/// [AdventurerInfo] trait's implementation if there are action points left or
/// nothing/false if they are zero.
///
/// For the specific implementations, please see the corresponding adventurer's
/// module for further information.
pub trait Adventurer: AdventurerInfo + Send + Sync {
    /// Checks if the adventurer still has something they can do this turn and
    /// returns true if so, false if they cannot do anything except end
    /// their turn.
    fn can_act(&self, act_points: u8) -> bool { act_points != 0 }

    fn moves(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            AdventurerInfo::moves(self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn special_moves(&self, _map: &FullMap, _act_points: u8) -> Vec<Vec2<u8>> { unimplemented!() }

    /// If something special should happen, when the adventurer moves, this
    /// needs to be implemented. Does not get the action points, since none
    /// of the standard adventurers have extra movements per action point.
    fn on_move(&mut self) {}

    fn can_move_other(&self, _act_points: u8) -> bool { false }

    /// When the adventurer has moved another this is called. Must be
    /// implemented, if something special should happen, like with the
    /// Navigator, which has to handle their extra push.
    fn on_move_other(&mut self, _act_points: &mut u8) {}

    fn drains(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            AdventurerInfo::drains(self, &map)
        }
        else {
            Vec::new()
        }
    }

    /// If something special happens, the adventurer must implement this
    /// function. Default implementation is to subtract one from the action
    /// points.
    fn on_drain(&mut self, act_points: &mut u8) { *act_points -= 1; }

    fn can_trade_with(&self, other: &dyn Positionable, act_points: u8) -> bool {
        if act_points != 0 {
            AdventurerInfo::can_trade_with(self, other)
        }
        else {
            false
        }
    }
}

/// Information about the adventurer, that is known and true no matter the
/// current action state, but not necessarily known at compile time.
pub trait AdventurerInfo: Positionable {
    /// The normal movement set the adventurer can do. This usually does not
    /// have to be overwritten, since the normal movements should be the
    /// same.
    // TODO: This should be in the character, since it does not change depending on
    // TODO  the adventurer anymore.
    fn moves(&self, map: &FullMap) -> Vec<Vec2<u8>> {
        self.pos()
            .neighbours(Some(map.limit_rect()))
            .into_iter()
            .filter(|&pos| map.is_standable(pos))
            .collect()
    }

    /// The special move set of the adventurer. If the adventurer has a special
    /// ability that is not concerned with movement, the standard
    /// implementation will suffice.
    fn special_moves(&self, _map: &FullMap) -> Vec<Vec2<u8>> { Vec::new() }

    /// The position set the adventurer can drain from their current position.
    fn drains(&self, map: &FullMap) -> Vec<Vec2<u8>> {
        let mut positions = self.pos().neighbours(Some(map.limit_rect()));
        positions.push(self.pos());
        positions
            .into_iter()
            .filter(|&pos| map.is_standable(pos))
            .collect()
    }

    /// Returns if the Adventurer can trade cards to the one provided as other.
    fn can_trade_with(&self, other: &dyn Positionable) -> bool { self.pos() == other.pos() }
}
