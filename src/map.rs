//! Mapse contain the playing field of adventure island.
//!
//! A valid map contains 24 tiles that are connected in such a way, that the
//! standard moveset of up, down, left and right suffices to walk the entire
//! island by foot.

use crate::iter_2d::Iter2d;
use crate::math::{Rect, Vec2};
use std::ops::{Deref, DerefMut};

/// Mapse, that can be played on contain 24 unique island tiles.
pub type Full = Map<Option<IslandTile>>;

/// The black and white map is the raw map data that just saves, where island
/// tiles should be placed (true) and where the sea is (false). They are
/// usually used to generate maps randomly.
pub type BlackWhite = Map<bool>;

/// Alias for the position type an item must hold in order to be placeable on a
/// map.
pub type FieldPos = Vec2<u8>;

#[derive(Clone, Debug)]
pub struct Map<T> {
    data: Vec<Vec<T>>
}

pub trait MapExt {
    /// Get the [Rect](Rect<u8>) this map is bounded by, meaning that beyond it,
    /// there is only water and therefore it is considered out of bounds for
    /// anything that is placeable on the island. Does not mean all tiles
    /// inside it are island tiles, since not all maps are rectangular.
    fn limit_rect(&self) -> Rect<u8>;

    /// Checks, if a positionable object may be placed on the [FieldPos]
    /// provided, which means it returns false in case there is no island
    /// tile or it's gone.
    fn is_standable(&self, _pos: FieldPos) -> bool;
}

/// The different states an island can be.
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct IslandTile {
    state: IslandTileState
}

impl<T> Map<T> {
    /// Create a new Map and fill its contents with the fill_value.
    pub fn new(size: Vec2<u8>, fill_value: T) -> Self
    where
        T: Clone
    {
        Self {
            data: vec![vec![fill_value; size.x as usize]; size.y as usize]
        }
    }

    /// Get the item at the provided position of the map or `None`, if there is
    /// no item at the position.
    pub fn get(&self, pos: FieldPos) -> Option<&T> {
        if let Some(line) = self.data.get(pos.y as usize) {
            line.get(pos.x as usize)
        }
        else {
            None
        }
    }

    /// Set the item at the provided position of the map. Returns the item that
    /// was there before.
    ///
    /// # Panics
    /// If the index is out of bounds. The map will not be resized in this
    /// function.
    pub fn set(&mut self, pos: FieldPos, new: T) -> T {
        assert!(pos.x < self.width());
        assert!(pos.y < self.height());

        // Push the new item to the end of the line in question and swap it with the
        // element at the x position afterwards.
        self.data[pos.y as usize].push(new);
        self.data[pos.y as usize].swap_remove(pos.x as usize)
    }

    /// Iterator over all map tiles.
    pub fn iter(&self) -> Iter2d<T> { Iter2d::new(&self.data) }

    /// Amount of tiles in the x-direction.
    pub fn width(&self) -> u8 {
        if self.data.len() == 0 {
            0
        }
        else {
            self.data[0].len() as u8
        }
    }

    /// Amount of tiles in the y-direction.
    pub fn height(&self) -> u8 { self.data.len() as u8 }

    /// Amount of tiles in the x and y-direction.
    pub fn size(&self) -> Vec2<u8> { Vec2::from_values(self.width(), self.height()) }
}

impl<T> From<Vec<Vec<T>>> for Map<T> {
    fn from(from: Vec<Vec<T>>) -> Self {
        #[cfg(Debug)]
        {
            // The map can only be constructed, if all lines of the raw data are the same
            // length, since this function does not fill empty space.
            let len = if let Some(line) = from.get(0) {
                line.len()
            }
            else {
                0
            };
            assert_eq!(true, from.iter().all(|line| line.len() == len));
        }

        Self { data: from }
    }
}

impl<T> Deref for Map<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target { &self.data }
}
impl<T> DerefMut for Map<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }
}

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

impl IslandTile {
    pub fn state(&self) -> IslandTileState { self.state }

    pub fn set_state(&mut self, state: IslandTileState) { self.state = state; }
}
