use amethyst::prelude::*;

pub struct Settings {}

impl SimpleState for Settings {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
