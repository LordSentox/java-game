use amethyst::prelude::*;

/// The state representing the High Scores window
pub struct HighScores {}

impl SimpleState for HighScores {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
