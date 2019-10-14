use amethyst::prelude::*;

pub struct Settings {}

impl SimpleState for Settings {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {}
}
