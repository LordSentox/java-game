use std::ops::{Add, AddAssign};
use std::u8;

use crate::difficulty::Difficulty;

pub const MAX_WATER_LEVEL: u8 = 9;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WaterLevel {
    level: u8
}

impl WaterLevel {
    pub fn new(level: u8) -> Self { Self { level } }

    pub fn from_difficulty(difficulty: Difficulty) -> Self {
        let level = match difficulty {
            Difficulty::Novice => 0,
            Difficulty::Normal => 1,
            Difficulty::Elite => 2,
            Difficulty::Legendary => 3
        };

        Self::new(level)
    }

    pub fn draw_amount(&self) -> u8 {
        match self.level {
            0 | 1 => 2,
            2..=4 => 3,
            5 | 6 => 4,
            7 | 8 => 5,
            _ => u8::MAX
        }
    }
}

impl Add<u8> for WaterLevel {
    type Output = Self;

    fn add(self, other: u8) -> Self::Output { Self::new(self.level + other) }
}

impl AddAssign<u8> for WaterLevel {
    fn add_assign(&mut self, other: u8) { self.level += other; }
}
