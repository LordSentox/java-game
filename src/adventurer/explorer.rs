//! # The explorer adventurer
//!
//! # Special ability
//! The explorer can move diagonally

use super::{Adventurer, AdventurerInfo};
use crate::map::{Full as MapFull, MapExt};
use crate::positionable::Positionable;

#[derive(Positionable)]
pub struct Explorer {
    pos: FieldPos
}

impl Explorer {
    pub fn new() -> Self {
        Self {
            pos: FieldPos::new()
        }
    }

    pub fn implicit_special() -> bool { true }

    pub fn can_move_others() -> bool { false }
}

impl Adventurer for Explorer {}

impl AdventurerInfo for Explorer {
    /// Returns the diagonal movement set of the explorer.
    fn special_moves(&self, map: &MapFull) -> Vec<FieldPos> {
        let diagonal_neighbours = self.pos.diagonal_neighbours(Some(map.limit_rect()));

        diagonal_neighbours
            .into_iter()
            .filter(|&v| map.is_standable(v))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::{IslandTile, IslandTileInfo, IslandTileState};
    use crate::math::Vec2;

    #[test]
    fn special_moves() {
        let dry = IslandTile::new(IslandTileInfo::LostLagoon);
        let mut gone = IslandTile::new(IslandTileInfo::GoldGate);
        gone.set_state(IslandTileState::Gone);

        let mut map = MapFull::new(Vec2::from_values(5, 5), Some(dry));
        map.set(Vec2::from_values(1, 1), Some(gone.clone()));
        map.set(Vec2::from_values(1, 3), None);

        let mut explorer = Explorer::new();
        explorer.set_pos(Vec2::from_values(2, 2));

        let mut expected_moves = vec![Vec2::from_values(3, 1), Vec2::from_values(3, 3)];
        let mut actual_moves = AdventurerInfo::special_moves(&explorer, &map);

        expected_moves.sort();
        actual_moves.sort();

        assert_eq!(expected_moves, actual_moves);
    }
}
