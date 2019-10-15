use crate::math::{Rect, Vec2};

pub type Full = Map<IslandTile>;
pub type BlackWhite = Map<bool>;

pub struct Map<T> {
    tiles: Vec<Vec<Option<T>>>
}

pub enum IslandTileState {
    Dry,
    Flooded,
    Gone
}

pub struct IslandTile {
    state: IslandTileState
}

impl<T> Map<T> {
    pub fn limit_rect(&self) -> Rect<u8> { unimplemented!() }

    pub fn is_standable(&self, pos: Vec2<u8>) -> bool { unimplemented!() }
}
