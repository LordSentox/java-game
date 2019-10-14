use amethyst::ecs::{System, WriteStorage};

use crate::character::Character;

/// Checks if the Player wants to move to a point and does so, if the player can
/// move there without using their special ability.
pub struct MoveNormal {}

impl<'a> System<'a> for MoveNormal {
    type SystemData = (WriteStorage<'a, Character>);

    fn run(&mut self, (mut characters): Self::SystemData) { unimplemented!() }
}
