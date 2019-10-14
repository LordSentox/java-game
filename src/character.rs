use amethyst::ecs::{Component, DenseVecStorage};

use crate::math::Vec2;
use crate::specialist::Specialist;

#[derive(Component, Debug)]
pub struct Character<T: Spcialist> {
    pos: Vec2<u8>,
    internal_handler: T
}
