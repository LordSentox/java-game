use amethyst::prelude::*;

pub struct GamePreparations {}

impl SimpleState for GamePreparations {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {}
}
