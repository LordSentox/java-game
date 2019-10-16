use crate::map::Full as FullMap;
use crate::math::Vec2;
use crate::positionable::Positionable;

pub trait Adventurer: AdventurerInfo + Send + Sync {
    fn can_act(&self, act_points: u8) -> bool { act_points != 0 }

    fn moves(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            AdventurerInfo::moves(self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn special_moves(&self, _map: &FullMap, _act_points: u8) -> Vec<Vec2<u8>> { Vec::new() }

    fn on_move(&mut self, _act_points: &mut u8) {}

    fn can_move_other(&self, _act_points: u8) -> bool { false }

    fn on_move_other(&mut self, _act_points: &mut u8) {}

    fn drains(&self, map: &FullMap, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            AdventurerInfo::drains(self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn on_drain(&mut self, _act_points: &mut u8) {}

    fn can_transfer_card(&self, other: &dyn Positionable, act_points: u8) -> bool {
        if act_points != 0 {
            AdventurerInfo::can_transfer_cards(self, other)
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
    /// imlpmentation will suffice.
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

    // TODO: This might have to be rethought, because of the way one can trade cards
    // TODO  if one has too many. In that case the position or adventurer type does
    // TODO  not matter.
    fn can_transfer_cards(&self, other: &dyn Positionable) -> bool { self.pos() == other.pos() }
}
