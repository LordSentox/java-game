use amethyst::ecs::{System, WriteStorage};

use crate::character::Character;

pub struct MoveNormal {}

impl<'a> System<'a> for MoveNormal {
    type SystemData = (WriteStorage<'a, Character>);

    fn run(&mut self, (mut characters): Self::SystemData) { unimplemented!() }
}
