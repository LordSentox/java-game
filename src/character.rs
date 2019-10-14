use amethyst::ecs::{Component, DenseVecStorage};

use crate::math::Vec2;

#[derive(Component, Debug)]
pub struct Character {
    pos: Vec2<u8>
}
