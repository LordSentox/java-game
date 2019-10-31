//! # The diver adventurer
//!
//! # Special ability
//! The diver can swim through flooded or gone tiles, but only if there is or
//! was an island tile before. They can swim in all primary directions (Up,
//! Down, Left, Right) until they land on the island tile they want to end or
//! the first non-flooded non-gone tile.

use super::{Adventurer, AdventurerInfo};
use crate::bfs::bfs;
use crate::map::Full as MapFull;
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Diver {
    pos: FieldPos
}

impl Diver {
    pub fn new() -> Self {
        Self {
            pos: FieldPos::new()
        }
    }

    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Diver {}

impl AdventurerInfo for Diver {
    /// The diving move set on the map. Returns all positions the Diver can dive
    /// to and stand on after the action.
    fn special_moves(&self, map: &MapFull) -> Vec<FieldPos> {
        #[derive(Clone, Copy, PartialEq, Debug)]
        enum Marker {
            // The Diver may stand here after moving, but can also continue swimming
            EndPos,
            // The Diver may stand here after moving, but it is a dry tile, so they cannot
            // continue swimming
            EndPosNoSwimThrough,
            // The starting position. Diver may move to non-dry island tiles from here, but cannot
            // end here.
            // XXX: No way of knowing which marker was on a tile before. This should change.
            StartPos,
            // Tile is gone, but the Diver may swim through to other positions. They cannot end
            // here, though.
            SwimThroughOnly
        }

        use crate::map::IslandTileState::*;
        use Marker::*;

        let bfs = bfs(
            &map,
            Some(self.pos),
            StartPos,
            |(from, &source_marker), (_, tile)| {
                println!(
                    "Marking from {:?} ({:?}) to {:?}",
                    from, source_marker, tile
                );
                if let (Some(source_marker), Some(tile)) = (source_marker, tile) {
                    match (source_marker, tile.state()) {
                        // Coming from a water tile to a solid island, the diver may move here.
                        (EndPos, Dry) | (SwimThroughOnly, Dry) => Some(EndPosNoSwimThrough),
                        // Coming from a water tile or the starting point to a flooded island where
                        // an adventurer can stand, the diver may move here and continue moving, if
                        // they want to.
                        (StartPos, Flooded) | (EndPos, Flooded) | (SwimThroughOnly, Flooded) => {
                            println!("Marking endpos");
                            Some(EndPos)
                        }
                        // Coming from a water tile or the starting position to a gone tile, the
                        // diver may move through the water to any adjacent tile, but cannot end
                        // here.
                        (StartPos, Gone) | (EndPos, Gone) | (SwimThroughOnly, Gone) => {
                            println!("Marking swimthrough");
                            Some(SwimThroughOnly)
                        }
                        _ => None
                    }
                }
                else {
                    None
                }
            },
            |_, &marker| match marker {
                Some(EndPos) | Some(EndPosNoSwimThrough) => true,
                _ => false
            }
        )
        .expect("Unable to perform bfs for Diver");

        bfs.iter()
            .filter_map(|(pos, &reachable)| {
                if reachable {
                    Some(pos)
                }
                else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Diver;
    use crate::adventurer::AdventurerInfo;
    use crate::map::{Full as MapFull, IslandTile, IslandTileState};
    use crate::math::Vec2;

    #[test]
    fn special_moves() {
        let mut dry = IslandTile::new();
        dry.set_state(IslandTileState::Dry);
        let mut flooded = IslandTile::new();
        flooded.set_state(IslandTileState::Flooded);
        let mut gone = IslandTile::new();
        gone.set_state(IslandTileState::Gone);

        let map: MapFull = vec![
            vec![None, None, Some(dry.clone()), None, None],
            vec![None, None, Some(gone.clone()), None, None],
            vec![
                None,
                None,
                Some(flooded.clone()),
                Some(dry.clone()),
                Some(dry.clone()),
            ],
            vec![
                Some(dry.clone()),
                Some(flooded.clone()),
                Some(gone.clone()),
                None,
                Some(gone.clone()),
            ],
            vec![Some(dry.clone()), None, None, None, Some(flooded.clone())],
        ]
        .into();

        let diver = Diver::new(Vec2::from_values(0, 3));
        assert_eq!(
            vec![
                Vec2::from_values(2, 0),
                Vec2::from_values(2, 2),
                Vec2::from_values(3, 2),
                Vec2::from_values(0, 3),
                Vec2::from_values(1, 3)
            ],
            diver.special_moves(&map)
        )
    }
}
