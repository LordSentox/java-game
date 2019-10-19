//! Mapse contain the playing field of adventure island.

use crate::math::{Rect, Vec2};
use std::ops::{Deref, DerefMut};

/// Mapse, that can be played on contain 24 unique island tiles.
pub type Full = Map<IslandTile>;

/// The black and white map is the raw map data that just saves, where island
/// tiles should be placed (true) and where the sea is (false). They are
/// usually used to generate maps randomly.
pub type BlackWhite = Map<bool>;

/// Alias for the position type an item must hold in order to be placeable on a
/// map.
pub type FieldPos = Vec2<u8>;

/// A valid map contains 24 tiles that are connected in such a way, that the
/// standard moveset of up, down, left and right suffices to walk the entire
/// island by foot.
pub struct Map<T> {
    // Raw tile data. The outer [Vec] for the lines, the inner [Vec] for the colums
    tiles: Vec<Vec<Option<T>>>
}

/// The different states an island can be.
pub enum IslandTileState {
    /// The island is dry. Players can freely do anything on these tiles.
    Dry,
    /// Flooded tiles can be drained. Players can still freely do anything on
    /// these tiles, however they should be careful since they will be
    /// [Gone](IslandTileState::Gone) the next time their flood card is
    /// drawn.
    Flooded,
    /// The tile is gone. The diver can still swim through it, but other than
    /// that it's unusable.
    Gone
}

/// Represents one of 24 island map tiles.
pub struct IslandTile {
    state: IslandTileState
}

impl<T> Map<T> {
    /// Get the [Rect](Rect<u8>) this map is bounded by, meaning that beyond it,
    /// there is only water and therefore it is considered out of bounds for
    /// anything that is placeable on the island. Does not mean all tiles
    /// inside it are island tiles, since not all maps are rectangular.
    pub fn limit_rect(&self) -> Rect<u8> { unimplemented!() }

    /// Checks, if a positionable object may be placed on the [FieldPos]
    /// provided, which means it returns false in case there is no island
    /// tile or it's gone.
    pub fn is_standable(&self, _pos: FieldPos) -> bool { unimplemented!() }
}

impl<T> Deref for Map<T> {
    type Target = Vec<Vec<Option<T>>>;

    fn deref(&self) -> &Self::Target { &self.tiles }
}

impl<T> DerefMut for Map<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.tiles }
}
