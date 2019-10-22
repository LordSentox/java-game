//! Mapse contain the playing field of adventure island.
//!
//! A valid map contains 24 tiles that are connected in such a way, that the
//! standard moveset of up, down, left and right suffices to walk the entire
//! island by foot.

pub mod black_white;
pub use self::black_white::*;

pub mod full;
pub use self::full::*;

pub mod generate;

pub mod island_tile;
pub use self::island_tile::*;

use crate::iter_2d::Iter2d;
use crate::math::{Rect, Vec2};
use std::ops::{Deref, DerefMut};

/// Alias for the position type an item must hold in order to be placeable on a
/// map.
pub type FieldPos = Vec2<u8>;

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

#[derive(Clone, Debug)]
pub struct Map<T> {
    data: Vec<Vec<T>>
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
