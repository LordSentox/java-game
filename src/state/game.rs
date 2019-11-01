use amethyst::{core::Transform, prelude::*, renderer::Camera};

use crate::adventurer::AdventurerType;
use crate::asset;
use crate::character::Character;
use crate::map::{FieldPosComp, Full as MapFull, IslandTile, IslandTileInfo};
use crate::math::Vec2;
use std::mem;

pub const GAME_WIDTH: f32 = 1920.;
pub const GAME_HEIGHT: f32 = 1080.;

/// The state the game is in while a game is being played
pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<FieldPosComp>();
        world.register::<Character>();

        let character_sprite_sheet =
            asset::load_sprite_sheet("characters.png", "characters.ron", world);

        let mut map = MapFull::new(Vec2::from_values(6, 4), None);
        map.transform_mut().set_translation_xyz(64., 64., -1.);
        for y in 0..4 {
            for x in 0..6 {
                let tile_info: IslandTileInfo = unsafe { mem::transmute(y * 6 + x) };
                map.set(Vec2::from_values(x, y), Some(IslandTile::new(tile_info)));
            }
        }
        map.create_tile_entities(world);

        let test_courier = Character::new(AdventurerType::Courier);
        test_courier.spawn_entity(&map, world, character_sprite_sheet);
        init_camera(world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}
