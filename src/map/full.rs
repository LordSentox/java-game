//! Mapse, that can be played on contain 24 unique island tiles.
use super::{FieldPos, IslandTile, IslandTileState, Map, MapExt};
use crate::math::Rect;

pub type Full = Map<Option<IslandTile>>;

impl MapExt for Full {
    fn is_standable(&self, pos: FieldPos) -> bool {
        if let Some(Some(tile)) = self.get(pos) {
            tile.state() != IslandTileState::Gone
        }
        else {
            false
        }
    }

    fn limit_rect(&self) -> Rect<u8> {
        // Find the minimum rectangular hull around the island tiles and return it.
        let mut min_pos = FieldPos::new();
        let mut max_pos = FieldPos::new();

        self.iter().for_each(|(pos, e)| {
            if let Some(_) = e {
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
