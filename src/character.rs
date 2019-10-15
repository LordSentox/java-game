use amethyst::ecs::{Component, DenseVecStorage};

use crate::adventurer::Adventurer;

#[derive(Component)]
pub struct Character {
    internal_handler: Box<dyn Adventurer>
}
