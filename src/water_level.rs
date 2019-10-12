use std::ops::{Add, AddAssign};

pub const MAX_WATER_LEVEL: u8 = 9;

pub struct WaterLevel {
    level: u8
}

impl WaterLevel {
    pub fn new(level: u8) -> WaterLevel {
        WaterLevel {
            level
        }
    }

    pub fn from_difficulty(difficulty: Difficulty) -> WaterLevel {
        unimplemented!()
    }

    pub fn draw_amount(&self) -> u8 {
        match self.level {
            0 | 1 => 2,
            2..=4 => 3,
            5 | 6 => 4,
            7 | 8 => 5,
            _ => u8::max()
        }
    }
}

impl Add<u8> for WaterLevel {
    type Output = WaterLevel;

    fn add(&self, other: u8) -> Output {
        WaterLevel::new(self.level + other)
    }
}

impl AddAssign<u8> for WaterLevel {
    fn add_assign(&mut self, other: u8) {
        self.level += other;
    }
}