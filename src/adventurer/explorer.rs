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
