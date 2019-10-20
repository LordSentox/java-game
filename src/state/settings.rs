use amethyst::prelude::*;

/// The state representing the settings window
/// Music and Effect volume can be changed, and Developer Tools disabled or enabled here
pub struct Settings {}

impl SimpleState for Settings {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
