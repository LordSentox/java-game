use amethyst::{core::Transform, prelude::*, renderer::Camera};

use crate::map::{FieldPosComp, Full as MapFull};
use crate::math::Vec2;

pub const GAME_WIDTH: f32 = 1920.;
pub const GAME_HEIGHT: f32 = 1080.;

/// The state the game is in while a game is being played
pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<MapFull>();
        world.register::<FieldPosComp>();

        let map = MapFull::new(Vec2::from_values(10, 7), None);
        map.into_entity(Vec2::from_values(64., 64.), world);

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
