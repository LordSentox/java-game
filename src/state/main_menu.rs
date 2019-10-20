use amethyst::prelude::*;

/// The state representing the main menu
pub struct MainMenu {}

impl SimpleState for MainMenu {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
