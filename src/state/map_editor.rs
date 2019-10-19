use amethyst::prelude::*;

/// The state representing the map editor
pub struct MapEditor {}

impl SimpleState for MapEditor {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
