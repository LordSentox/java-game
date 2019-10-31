use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Builder, Component, DenseVecStorage, Entity, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet, Transparent}
};

use crate::adventurer::{Adventurer, AdventurerType};
use crate::map::Full as MapFull;
use std::ops::{Deref, DerefMut};

/// A character is the playable instance of one player.
///
/// # Associated Components
/// [Transform](amethyst::core::Transform) - The position of the character on
/// the screen. Automatically updated by the
/// [CharacterTransformUpdate](crate::system::CharacterTransformUpdate) system.
#[derive(Component)]
pub struct Character {
    internal_handler: Box<dyn Adventurer>,
    adventurer_type:  AdventurerType
}

impl Character {
    pub fn spawn_entity(
        mut self,
        map: &MapFull,
        world: &mut World,
        sprite_sheet: Handle<SpriteSheet>
    ) -> Entity {
        // Position the character onto the map correctly it the adventurers spawn point
        let pos = map.spawn_point(&self.adventurer_type);
        self.set_pos(pos);

        let renderer = SpriteRender {
            sprite_sheet,
            sprite_number: self.adventurer_type as usize
        };

        world
            .create_entity()
            .with(Transform::default())
            .with(self)
            .with(renderer)
            .with(Transparent)
            .build()
    }
}

impl Deref for Character {
    type Target = dyn Adventurer;

    fn deref(&self) -> &Self::Target { self.internal_handler.as_ref() }
}

impl DerefMut for Character {
    fn deref_mut(&mut self) -> &mut Self::Target { self.internal_handler.as_mut() }
}
