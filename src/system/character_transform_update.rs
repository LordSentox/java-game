use amethyst::{
    core::Transform,
    ecs::{Entity, Join, ReadStorage, System, WriteStorage}
};

use crate::character::Character;
use crate::map::full::calculate_field_translation;

/// Responsible for updating the transformations of the player characters based
/// on the current position of them and the information available of the map
/// they are playing on.
pub struct CharacterTransformUpdate {
    map: Entity
}

impl<'a> System<'a> for CharacterTransformUpdate {
    type SystemData = (ReadStorage<'a, Character>, WriteStorage<'a, Transform>);

    fn run(&mut self, (characters, mut transforms): Self::SystemData) {
        // TODO: Can this be done without cloning the transform in safe rust?
        let map_transform = transforms
            .get(self.map)
            .expect("Map has no transform. This is a bug.")
            .clone();

        for (character, transform) in (&characters, &mut transforms).join() {
            *transform = calculate_field_translation(&map_transform, character.pos());
        }
    }
}
