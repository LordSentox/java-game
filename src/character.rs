use amethyst::ecs::{Component, DenseVecStorage};

use crate::math::Vec2;
use crate::specialist::{Specialist, SpecialistInfo};

#[derive(Component)]
pub struct Character {
    pos: Vec2<u8>,
    internal_handler: Box<dyn Specialist>
}
