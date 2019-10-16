use amethyst::prelude::*;

pub struct MainMenu {}

impl SimpleState for MainMenu {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
