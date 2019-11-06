//! # The engineer adventurer
//!
//! # Special ability
//! The engineer can shore up two tiles for one action point, unless they
//! perform a movement action in between those two drains.

use super::{Adventurer, AdventurerInfo};
use crate::map::Full as MapFull;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Engineer {
    pos: FieldPos,
    extra_drain: bool
}

impl Engineer {
    pub fn new() -> Self {
        Self {
            pos: FieldPos::new(),
            extra_drain: false
        }
    }

    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Engineer {
    /// If the engineer moves, they may not drain a second tile without spending
    /// an extra action point.
    fn on_move(&mut self) { self.extra_drain = false; }

    /// Instead of just checking the action points, this function must also
    /// check if there is an extra drain, in which case the action points
    /// can be ignored.
    fn drains(&self, map: &MapFull, act_points: u8) -> Vec<FieldPos> {
        if act_points != 0 || self.extra_drain {
            AdventurerInfo::drains(self, &map)
        }
        else {
            Vec::new()
        }
    }

    /// Instead of always costing an action point. If for the last drain action
    /// an action point was already consumed, the action points remain
    /// unchanged.
    fn on_drain(&mut self, act_points: &mut u8) {
        if self.extra_drain {
            self.extra_drain = false;
        }
        else {
            *act_points -= 1;
            self.extra_drain = true;
        }
    }
}

impl AdventurerInfo for Engineer {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::{Full as MapFull, IslandTile, IslandTileInfo, IslandTileState};
    use crate::math::Vec2;

    fn setup_drain_map() -> MapFull {
        let mut flooded_tile = IslandTile::new(IslandTileInfo::TempleOfTheSun);
        flooded_tile.set_state(IslandTileState::Flooded);

        MapFull::new(Vec2::from_values(1, 1), Some(flooded_tile))
    }

    #[test]
    fn drain_twice_correct_consume_amount() {
        let mut engineer = Engineer::new();
        let map = setup_drain_map();

        let mut action_points = 2;
        assert_eq!(
            vec![Vec2::from_values(0, 0)],
            Adventurer::drains(&engineer, &map, action_points)
        );
        Adventurer::on_drain(&mut engineer, &mut action_points);
        assert_eq!(
            vec![Vec2::from_values(0, 0)],
            Adventurer::drains(&engineer, &map, action_points)
        );
        assert_eq!(1, action_points);
        Adventurer::on_drain(&mut engineer, &mut action_points);
        assert_eq!(
            vec![Vec2::from_values(0, 0)],
            Adventurer::drains(&engineer, &map, action_points)
        );
        assert_eq!(1, action_points);
    }

    #[test]
    fn drain_twice_one_action_point() {
        let mut engineer = Engineer::new();
        let map = setup_drain_map();

        let mut action_points = 1;
        assert_eq!(
            vec![Vec2::from_values(0, 0)],
            Adventurer::drains(&engineer, &map, action_points)
        );
        Adventurer::on_drain(&mut engineer, &mut action_points);
        assert_eq!(
            vec![Vec2::from_values(0, 0)],
            Adventurer::drains(&engineer, &map, action_points)
        );
        assert_eq!(0, action_points);
        Adventurer::on_drain(&mut engineer, &mut action_points);
        assert_eq!(
            Vec::<FieldPos>::new(),
            Adventurer::drains(&engineer, &map, action_points)
        );
        assert_eq!(0, action_points);
    }

    #[test]
    fn move_denies_extra_drain() {
        let mut engineer = Engineer::new();
        let map = setup_drain_map();

        let mut action_points = 1;
        assert_eq!(
            vec![Vec2::from_values(0, 0)],
            Adventurer::drains(&engineer, &map, action_points)
        );
        Adventurer::on_drain(&mut engineer, &mut action_points);
        Adventurer::on_move(&mut engineer);
        assert_eq!(
            Vec::<FieldPos>::new(),
            Adventurer::drains(&engineer, &map, action_points)
        );
    }
}
