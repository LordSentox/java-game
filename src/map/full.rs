//! Mapse, that can be played on contain 24 unique island tiles.

use super::{FieldPos, IslandTile, IslandTileState, Map, MapExt, TILE_AMOUNT};
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
    pub fn into_entity(self, pos: Vec2<f32>, world: &mut World) -> Entity {
        let sprites = asset::load_sprite_sheet("tiles.png", "tiles_sheet.ron", world);

        let mut sprite = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let mut transform = Transform::default();
                transform.set_scale(Vector3::new(TILE_SCALE, TILE_SCALE, TILE_SCALE));
                transform.set_translation_xyz(
                    pos.x + TILE_WIDTH * x as f32 * TILE_SCALE,
                    pos.y + TILE_HEIGHT * y as f32 * TILE_SCALE,
                    0.0
                );

                let sprite_render = SpriteRender {
                    sprite_sheet:  sprites.clone(),
                    sprite_number: sprite % TILE_AMOUNT as usize
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

/// Graphics helper function to convert the field position in a map to the
/// actual screen position to the tile.
///
/// # Parameters
/// `map_transform` - The base transform of the map, i.e. where the map is on
/// the screen `field_pos` - The tile position on the map
///
/// # Returns
/// Transform that should be used for the thing on the map
pub fn calculate_field_translation(map_transform: &Transform, field_pos: FieldPos) -> Transform {
    let mut transform = map_transform.clone();
    transform.prepend_translation_x(TILE_WIDTH * field_pos.x as f32 * TILE_SCALE);
    transform.prepend_translation_y(TILE_HEIGHT * field_pos.y as f32 * TILE_SCALE);

    transform
}
