//! Traits that describe the actions an Adventurer can take that may be unique
//! to them.

use crate::map::{Full as MapFull, IslandTileState, MapExt};
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

    fn moves(&self, map: &MapFull, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            AdventurerInfo::moves(self, &map)
        }
        else {
            Vec::new()
        }
    }

    fn special_moves(&self, map: &MapFull, act_points: u8) -> Vec<Vec2<u8>> {
        if act_points != 0 {
            AdventurerInfo::special_moves(self, &map)
        }
        else {
            Vec::new()
        }
    }

    /// If something special should happen, when the adventurer moves, this
    /// needs to be implemented. Does not get the action points, since none
    /// of the standard adventurers have extra movements per action point.
    fn on_move(&mut self) {}

    fn can_move_other(&self, _act_points: u8) -> bool { false }

    /// When the adventurer has moved another this is called. Must be
    /// implemented, if something special should happen, like with the
    /// Navigator, which has to handle their extra push.
    fn on_move_other(&mut self, _act_points: &mut u8) {
        panic!("Player moved someone else, but should not have that ability.")
    }

    fn drains(&self, map: &MapFull, act_points: u8) -> Vec<Vec2<u8>> {
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
    fn moves(&self, map: &MapFull) -> Vec<Vec2<u8>> {
        self.pos()
            .neighbours(Some(map.limit_rect()))
            .into_iter()
            .filter(|&pos| map.is_standable(pos))
            .collect()
    }

    /// The special move set of the adventurer. If the adventurer has a special
    /// ability that is not concerned with movement, the standard
    /// implementation will suffice.
    fn special_moves(&self, _map: &MapFull) -> Vec<Vec2<u8>> { Vec::new() }

    /// The position set the adventurer can drain from their current position.
    fn drains(&self, map: &MapFull) -> Vec<Vec2<u8>> {
        let mut positions = self.pos().neighbours(Some(map.limit_rect()));
        positions.push(self.pos());
        positions
            .into_iter()
            .filter(|&pos| {
                if let Some(tile) = map.get(pos).unwrap() {
                    tile.state() == IslandTileState::Flooded
                }
                else {
                    false
                }
            })
            .collect()
    }

    /// Returns if the Adventurer can trade cards to the one provided as other.
    fn can_trade_with(&self, other: &dyn Positionable) -> bool { self.pos() == other.pos() }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::{IslandTile, IslandTileInfo, IslandTileState};

    // Boring adventurer without any special abilities
    #[derive(Positionable)]
    struct CaptainAwesome {
        pos: FieldPos
    }

    impl Adventurer for CaptainAwesome {}
    impl AdventurerInfo for CaptainAwesome {}

    fn map_all_moveable(size: Vec2<u8>) -> MapFull {
        let dry = IslandTile::new(IslandTileInfo::CoralPalace);
        let mut flooded = dry.clone();
        flooded.set_state(IslandTileState::Flooded);

        let mut tiles = vec![Vec::with_capacity(size.x as usize); size.y as usize];
        for y in 0..size.y {
            for x in 0..size.x {
                tiles[y as usize].push(if (x + y) % 2 == 0 {
                    Some(dry.clone())
                }
                else {
                    Some(flooded.clone())
                });
            }
        }

        tiles.into()
    }

    #[test]
    fn default_moves_enough_act_points() {
        let map = map_all_moveable(Vec2::from_values(5, 5));
        let adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 2)
        };

        let mut expected_moves = vec![
            Vec2::from_values(2, 1),
            Vec2::from_values(2, 3),
            Vec2::from_values(1, 2),
            Vec2::from_values(3, 2),
        ];
        let mut actual_moves = Adventurer::moves(&adventurer, &map, 3);

        expected_moves.sort();
        actual_moves.sort();

        assert_eq!(expected_moves, actual_moves);
    }

    #[test]
    fn default_moves_no_act_points() {
        let map = map_all_moveable(Vec2::from_values(5, 5));
        let adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 2)
        };

        let actual_moves = Adventurer::moves(&adventurer, &map, 0);
        assert_eq!(Vec::<FieldPos>::new(), actual_moves);
    }

    #[test]
    fn can_act() {
        let adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 2)
        };

        assert!(adventurer.can_act(1));
        assert!(!adventurer.can_act(0));
    }

    #[test]
    fn special_moves() {
        // Special moves should by default be empty, so return an empty Vec if
        // the adventurer has action points as well as if they have
        // none.
        let map = map_all_moveable(Vec2::from_values(5, 5));
        let adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 2)
        };

        let actual_moves = Adventurer::special_moves(&adventurer, &map, 1);
        assert_eq!(Vec::<FieldPos>::new(), actual_moves);
        let actual_moves = Adventurer::special_moves(&adventurer, &map, 0);
        assert_eq!(Vec::<FieldPos>::new(), actual_moves);
    }

    #[test]
    fn drains() {
        let mut map = map_all_moveable(Vec2::from_values(5, 5));
        map.get_mut(FieldPos::from_values(2, 2))
            .unwrap()
            .as_mut()
            .unwrap()
            .set_state(IslandTileState::Gone);
        map.get_mut(FieldPos::from_values(1, 1))
            .unwrap()
            .as_mut()
            .unwrap()
            .set_state(IslandTileState::Flooded);
        let adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 1)
        };

        let mut expected_drains = vec![adventurer.pos(), FieldPos::from_values(1, 1)];
        let mut actual_drains = Adventurer::drains(&adventurer, &map, 1);

        expected_drains.sort();
        actual_drains.sort();

        assert_eq!(expected_drains, actual_drains);
        // Unable to drain, if there are no action points left
        assert_eq!(
            Vec::<FieldPos>::new(),
            Adventurer::drains(&adventurer, &map, 0)
        );
    }

    #[test]
    fn on_drain_sub_points() {
        let mut points = 2;
        let mut adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 1)
        };
        Adventurer::on_drain(&mut adventurer, &mut points);

        assert_eq!(1, points);
    }

    #[test]
    #[should_panic]
    fn on_drain_no_points() {
        let mut points = 0;
        let mut adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 1)
        };
        Adventurer::on_drain(&mut adventurer, &mut points);
    }

    #[test]
    #[should_panic]
    fn on_move_other() {
        let mut adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 2)
        };

        adventurer.on_move_other(&mut 1);
    }

    #[test]
    fn cover_untestable() {
        let mut adventurer = CaptainAwesome {
            pos: FieldPos::from_values(2, 2)
        };

        adventurer.on_move();
        assert!(!Adventurer::can_move_other(&adventurer, 3));
    }
}
