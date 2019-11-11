//! Mapse, that can be played on contain 24 unique island tiles.

use super::{FieldPos, IslandTile, IslandTileState, Map, MapExt, TILE_AMOUNT};
use crate::adventurer::AdventurerType;
use crate::asset;
use crate::math::{Rect, Vec2};
use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, World, WorldExt},
    prelude::*,
    renderer::SpriteRender
};
use nalgebra::Vector3;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Full {
    map: Map<Option<IslandTile>>,
    transform: Transform
}

pub const TILE_WIDTH: f32 = 400.;
pub const TILE_HEIGHT: f32 = 400.;
pub const TILE_SCALE: f32 = 0.32;

#[derive(Component)]
pub struct FieldPosComp(FieldPos);

impl MapExt for Map<Option<IslandTile>> {
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
            if e.is_some() {
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
    pub fn new(size: Vec2<u8>, fill_with: Option<IslandTile>) -> Self {
        Self {
            map: Map::new(size, fill_with),
            transform: Transform::default()
        }
    }

    pub fn transform(&self) -> &Transform { &self.transform }
    pub fn transform_mut(&mut self) -> &mut Transform { &mut self.transform }

    pub fn create_tile_entities(&self, world: &mut World) {
        let sprites = asset::load_sprite_sheet("tiles.png", "tiles_sheet.ron", world);

        let mut sprite = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let mut transform = self.transform.clone();
                transform.set_scale(Vector3::new(TILE_SCALE, TILE_SCALE, TILE_SCALE));
                transform.prepend_translation_x(TILE_WIDTH * x as f32 * TILE_SCALE);
                transform.prepend_translation_y(TILE_HEIGHT * y as f32 * TILE_SCALE);

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
    }

    /// Find the spawn point of the given adventurer type
    ///
    /// # Parameters
    /// `adventurer_type` - The type of the adventurer the spawn point is
    /// required of
    ///
    /// # Panics
    /// If the adventurer does not have a corresponding spawn tile. In a valid
    /// map, this should never happen, since if all tiles are present, all
    /// potential adventurers have spawn points.
    pub fn spawn_point(&self, adventurer_type: AdventurerType) -> FieldPos {
        self.iter()
            .find(|(_pos, tile)| {
                if let Some(tile) = tile {
                    match tile.info().player_spawn() {
                        Some(adventurer_spawn) => adventurer_type == adventurer_spawn,
                        _ => false
                    }
                }
                else {
                    false
                }
            })
            .expect("No Spawnpoint registered for the adventurer type")
            .0 // Return only the position, not the type
    }
}

impl From<Vec<Vec<Option<IslandTile>>>> for Full {
    fn from(from: Vec<Vec<Option<IslandTile>>>) -> Self {
        Self {
            map: from.into(),
            transform: Transform::default()
        }
    }
}

impl Default for Full {
    fn default() -> Self { Self::new(Vec2::from_values(10, 7), None) }
}

impl Deref for Full {
    type Target = Map<Option<IslandTile>>;

    fn deref(&self) -> &Self::Target { &self.map }
}

impl DerefMut for Full {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.map }
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
pub fn calculate_field_translation(map_transform: &Transform, field_pos: FieldPos) -> Vec2<f32> {
    let mut transform = map_transform.clone();
    transform.prepend_translation_x(TILE_WIDTH * field_pos.x as f32 * TILE_SCALE);
    transform.prepend_translation_y(TILE_HEIGHT * field_pos.y as f32 * TILE_SCALE);

    Vec2::from_values(transform.translation().x, transform.translation().y)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::IslandTileInfo;

    #[test]
    fn cover_untestable() {
        let mut map = Full::new(
            Vec2::from_values(4, 6),
            Some(IslandTile::new(IslandTileInfo::BreakersBridge))
        );

        map.transform();
        map.transform_mut();
    }
}
