use amethyst::ecs::System;

/// Responsible for updating the transformations of the player characters based
/// on the current position of them and the information available of the map
/// they are playing on.
pub struct CharacterTransformUpdate {}

impl<'a> System<'a> for CharacterTransformUpdate {
    type SystemData = ();

    fn run(&mut self, mut _characters: Self::SystemData) { unimplemented!() }
}
