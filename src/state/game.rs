use amethyst::prelude::*;

/// The state the game is in while a game is being played
pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
