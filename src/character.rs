use amethyst::ecs::{Component, DenseVecStorage};

use crate::adventurer::Adventurer;
use std::ops::{Deref, DerefMut};

/// A character is the playable instance of one player.
///
/// # Associated Components
/// [Transform](amethyst::core::Transform) - The position of the character on
/// the screen. Automatically updated by the
/// [CharacterTransformUpdate](crate::system::CharacterTransformUpdate) system.
#[derive(Component)]
pub struct Character {
    internal_handler: Box<dyn Adventurer>
}

impl Deref for Character {
    type Target = dyn Adventurer;

    fn deref(&self) -> &Self::Target { self.internal_handler.as_ref() }
}

impl DerefMut for Character {
    fn deref_mut(&mut self) -> &mut Self::Target { self.internal_handler.as_mut() }
}
