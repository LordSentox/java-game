//! Mapse, that can be played on contain 24 unique island tiles.

use super::{FieldPos, IslandTile, IslandTileState, Map, MapExt};
use crate::asset;
use crate::math::{Rect, Vec2};
use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender
};
use nalgebra::Vector3;

pub type Full = Map<Option<IslandTile>>;

pub const TILE_WIDTH: f32 = 400.;
pub const TILE_HEIGHT: f32 = 400.;
pub const TILE_SCALE: f32 = 0.32;

#[derive(Component)]
pub struct FieldPosComp(FieldPos);

impl MapExt for Full {
    fn is_standable(&self, pos: FieldPos) -> bool {
        if let Some(Some(tile)) = self.get(pos) {
            tile.state() != IslandTileState::Gone
        }
        else {
            false
        }
    }

    fn limit_rect(&self) -> Rect<u8> {
        // Find the minimum rectangular hull around the island tiles and return it.
        let mut min_pos = FieldPos::new();
        let mut max_pos = FieldPos::new();

        self.iter().for_each(|(pos, e)| {
            if let Some(_) = e {
                min_pos.x = min_pos.x.min(pos.x);
                min_pos.y = min_pos.y.min(pos.y);
                max_pos.x = max_pos.x.max(pos.x);
                max_pos.y = max_pos.y.max(pos.y);
            }
        });

        Rect::from_slice([
            min_pos.x,
            min_pos.y,
            max_pos.x - min_pos.x,
            max_pos.y - min_pos.y
        ])
    }
}

impl Full {
    pub fn into_rendered(self, pos: Vec2<f32>, world: &mut World) -> Entity {
        let sprites_dry = asset::load_sprite_sheet("tiles_dry.png", "tiles_sheet.ron", world);
        //let _sprites_flooded =
        //    asset::load_sprite_sheet("tiles_flooded.png", "tiles_sheet.ron", world);

        let mut sprite = 0;
        for y in 0..self.width() {
            for x in 0..self.height() {
                let mut transform = Transform::default();
                transform.set_scale(Vector3::new(TILE_SCALE, TILE_SCALE, TILE_SCALE));
                transform.set_translation_xyz(
                    pos.x + TILE_WIDTH * x as f32 * TILE_SCALE,
                    pos.y + TILE_HEIGHT * y as f32 * TILE_SCALE,
                    0.0
                );

                let sprite_render = SpriteRender {
                    sprite_sheet:  sprites_dry.clone(),
                    sprite_number: sprite % 24
                };

                world
                    .create_entity()
                    .with(sprite_render.clone())
                    .with(FieldPosComp(FieldPos::from_values(x, y)))
                    .with(transform)
                    .build();

                sprite += 1;
            }
        }

        let mut transform = Transform::default();
        transform.set_translation_xyz(pos.x, pos.y, 0.0);

        world.create_entity().with(self).with(transform).build()
    }
}
