use std::ops::{Add, AddAssign};
use std::u8;

use crate::action_state::ActionState;
use crate::difficulty::Difficulty;

/// The water level above which (inclusive) the game is lost.
pub const LOOSING_WATER_LEVEL: u8 = 9;

/// The water or flooding level the players are currently facing. This
/// determines the amount of flood cards the players have to draw in the flood
/// card phase.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WaterLevel {
    level: u8
}

impl WaterLevel {
    /// Create a new WaterLevel from the integer level provided
    pub fn new(level: u8) -> Self { Self { level } }

    /// Create a starting WaterLevel from the Difficulty provided.
    pub fn from_difficulty(difficulty: Difficulty) -> Self {
        let level = match difficulty {
            Difficulty::Novice => 0,
            Difficulty::Normal => 1,
            Difficulty::Elite => 2,
            Difficulty::Legendary => 3
        };

        Self::new(level)
    }

    /// Create a new flood state. Can be used at the beginning of the flood
    /// phase to create the starting state for it. It can not be used to
    /// create the next state when a card has been taken from the flood card
    /// stack.
    pub fn create_flood_state(&self) -> ActionState {
        ActionState::DrawFloodCards(self.draw_amount())
    }

    /// Get the amount of cards that should be drawn in the flood phase with the
    /// water being at the level that it is. If the game has already been
    /// lost it returns a ridiculously high amount of cards that should be
    /// drawn.
    pub fn draw_amount(&self) -> u8 {
        match self.level {
            0 | 1 => 2,
            2..=4 => 3,
            5 | 6 => 4,
            7 | 8 => 5,
            _ => u8::MAX
        }
    }

    /// Check if the water level is too high. Returns true, if the game has been
    /// lost because of it, false otherwise. Note that this does not exclude
    /// the possibility of the game being lost because of other means.
    pub fn game_lost(&self) -> bool { self.level >= LOOSING_WATER_LEVEL }
}

/// Create a new WaterLevel which is incremented by the given amount. Can be
/// useful if one wants to know, what the level would be after drawing a
/// specific amount of flood cards, without actually changing the water level.
///
/// # Example
/// If you want to know if the game can still be continued, when you would draw
/// two floods rise cards you may want to use it like this:
///
/// ```
/// use water_level::WaterLevel;
///
/// // Create a new WaterLevel with the flood in state 6
/// let level = WaterLevel::new(6);
///
/// // Check, if the game could be continued with 2 waters rise cards drawn
/// assert!(!(level + 2).game_lost());
///
/// // Check, if the game could be continued with 3 waters rise cards drawn
/// assert!((level + 3).game_lost());
/// ```
impl Add<u8> for WaterLevel {
    type Output = Self;

    fn add(self, other: u8) -> Self::Output { Self::new(self.level + other) }
}

/// Let the WaterLevel rise by the amount provided.
///
/// # Example
/// If two flood rise cards have been drawn, you might want to use it like this:
///
/// ```
/// use water_level::WaterLevel;
///
/// // Create a new level with the flood in state two
/// let mut level = WaterLevel::new(2);
///
/// // Here, two waters rise cards would be drawn.
///
/// // Afterwards, the water level is incremented by two.
/// level += 2;
///
/// assert_eq!(WaterLevel::new(4), level);
/// ```
impl AddAssign<u8> for WaterLevel {
    fn add_assign(&mut self, other: u8) { self.level += other; }
}
