//! The black and white map is the raw map data that just saves, where island
//! tiles should be placed (true) and where the sea is (false). They are
//! usually used to generate maps randomly.

use super::{FieldPos, Map, MapExt};
use crate::math::Rect;

pub type BlackWhite = Map<bool>;

impl MapExt for BlackWhite {
    fn is_standable(&self, pos: FieldPos) -> bool {
        if let Some(tile) = self.get(pos) {
            *tile
        }
        else {
            false
        }
    }

    fn limit_rect(&self) -> Rect<u8> {
        // Find the minimum rectangular hull around all true tiles (island tiles) and
        // return it.
        let mut min_pos = FieldPos::new();
        let mut max_pos = FieldPos::new();

        self.iter().for_each(|(pos, &e)| {
            if e {
                min_pos.x = min_pos.x.min(pos.x);
                min_pos.y = min_pos.y.min(pos.y);
                max_pos.x = max_pos.x.max(pos.x);
                max_pos.y = max_pos.y.max(pos.y);
            }
        });

        Rect::from_slice([
            min_pos.x,
            min_pos.y,
            max_pos.x - min_pos.x,
            max_pos.y - min_pos.y
        ])
    }
}
