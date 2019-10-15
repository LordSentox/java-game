use crate::map::Full as FullMap;
use crate::math::Vec2;
use crate::positionable::Positionable;

pub trait Specialist: SpecialistInfo {
    fn can_act(&self, act_points: u8) -> bool { act_points != 0 }

    fn moves(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            SpecialistInfo::moves(&self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn special_moves(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> { Vec::new() }

    fn on_move(&mut self, act_points: &mut u8) {}

    fn can_move_other(&self, act_points: u8) -> bool { false }

    fn on_move_other(&mut self, act_points: &mut u8) {}

    fn drains(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            SpecialistInfo::drains(&self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn on_drain(&mut self, act_points: &mut u8) {}

    fn can_transfer_card<T: Positionable>(&self, other: &T, act_points: u8) -> bool {
        if act_points != 0 {
            SpecialistInfo::can_transfer_cards(&self, &other)
        }
        else {
            Vec::new()
        }
    }
}

/// Information about the specialist, that is known and true no matter the
/// current action state, but not necessarily known at compile time.
pub trait SpecialistInfo: Positionable {
    /// Returns true, if the player does not have to do anything to activate the
    /// special ability of the adventurer. This does not mean the
    /// special_moves function should be omitted when querying for all
    /// movement options.
    // TODO: This should be moved to the adventurer type enum, since it is known at
    // TODO  compile time.
    fn implicit_special() -> bool { false }

    /// The normal movement set the adventurer can do. This usually does not
    /// have to be overwritten, since the normal movements should be the
    /// same.
    // TODO: This should be in the character, since it does not change depending on
    // TODO  the adventurer anymore.
    fn moves(&self, map: &FullMap) -> Vec<Vec2<u8>> {
        self.pos()
            .neighbours(map.limit_rect())
            .iter()
            .filter(|pos| map.is_standable(pos))
            .collect()
    }

    /// The special move set of the adventurer. If the adventurer has a special
    /// ability that is not concerned with movement, the standard
    /// imlpmentation will suffice.
    fn special_moves(&self, map: &FullMap) -> Vec<Vec2<u8>> { Vec::new() }

    /// Returns if the player is in principle capable of moving others.
    // TODO: This should be moved to the adventurer type enum, since it is known at
    // TODO  compile time.
    fn can_move_others() -> bool { false }

    /// The position set the adventurer can drain from their current position.
    fn drains(&self, map: &FullMap) -> Vec<Vec2<u8>> {
        let mut positions = self.pos().neighbours(map.limit_rect());
        positions.push(self.pos());
        positions
            .iter()
            .filter(|pos| map.is_standable(pos))
            .collect()
    }

    // TODO: This might have to be rethought, because of the way one can trade cards
    // TODO  if one has too many. In that case the position or adventurer type does
    // TODO  not matter.
    fn can_transfer_cards<T: Positionable>(&self, other: &T) -> bool { self.pos() == other.pos() }
}
