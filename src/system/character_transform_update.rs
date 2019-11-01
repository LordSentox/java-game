use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage}
};

use crate::character::Character;
use crate::map::{full::calculate_field_translation, Full as MapFull};

/// Responsible for updating the transformations of the player characters based
/// on the current position of them and the information available of the map
/// they are playing on.
pub struct CharacterTransformUpdate;

impl<'a> System<'a> for CharacterTransformUpdate {
    type SystemData = (
        Read<'a, MapFull>,
        ReadStorage<'a, Character>,
        WriteStorage<'a, Transform>
    );

    fn run(&mut self, (map, characters, mut transforms): Self::SystemData) {
        let map_transform = map.transform();

        for (character, transform) in (&characters, &mut transforms).join() {
            let translation = calculate_field_translation(&map_transform, character.pos());
            transform.set_translation_xyz(translation.x, translation.y, 0.);
        }
    }
}
