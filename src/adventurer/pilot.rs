//! # The pilot adventurer
//!
//! # Special ability
//! Once a turn the pilot may use their special ability to fly to any island
//! fild they like. Unlike with a helicopter card, they may not take anybody
//! with them.

use super::{Adventurer, AdventurerInfo};
use crate::map::{Full as MapFull, MapExt};
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Pilot {
    pos: FieldPos
}

impl Pilot {
    pub fn new(pos: FieldPos) -> Self { Self { pos } }

    pub fn implicit_special() -> bool { false }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Pilot {}

impl AdventurerInfo for Pilot {
    /// The pilot may fly anywhere on the map where they can stand when they use
    /// their special ability.
    fn special_moves(&self, map: &MapFull) -> Vec<FieldPos> {
        map.iter()
            .filter_map(|(pos, _)| {
                if map.is_standable(pos) && pos != self.pos() {
                    Some(pos)
                }
                else {
                    None
                }
            })
            .collect()
    }
}

impl Default for Pilot {
    fn default() -> Self {
        Self {
            pos: FieldPos::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::{IslandTile, IslandTileInfo, IslandTileState};
    use crate::math::Vec2;

    #[test]
    fn special_moves() {
        let pilot = Pilot::default();
        let dry = IslandTile::new(IslandTileInfo::BreakersBridge);
        let mut flooded = IslandTile::new(IslandTileInfo::FoolsLanding);
        flooded.set_state(IslandTileState::Flooded);
        let mut gone = IslandTile::new(IslandTileInfo::PhantomRock);
        gone.set_state(IslandTileState::Gone);

        let map = vec![
            // Row one
            vec![
                Some(dry.clone()),
                Some(gone.clone()),
                Some(flooded.clone()),
                Some(dry.clone()),
            ],
            // Row two
            vec![None, None, Some(dry.clone()), Some(flooded.clone())],
        ]
        .into();

        let mut expected_moves = vec![
            Vec2::from_values(2, 0),
            Vec2::from_values(3, 0),
            Vec2::from_values(2, 1),
            Vec2::from_values(3, 1),
        ];
        let mut actual_moves = AdventurerInfo::special_moves(&pilot, &map);

        expected_moves.sort();
        actual_moves.sort();

        assert_eq!(expected_moves, actual_moves);
    }
}
