use amethyst::prelude::*;

/// The state representing the game preparations settings screen, that is opened before starting a game.
/// On this screen, a map and players can be selected
pub struct GamePreparations {}

impl SimpleState for GamePreparations {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
