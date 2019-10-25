use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::World,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture}
};

/// Load a RON-defined SpriteSheet from a texture file
///
/// # Parameters
/// `asset_name` - Name of the texture file to load, relative to the assets
/// folder path
/// `asset_conf` - Path of the RON-File where the SpriteSheet information is
/// defined
/// `world` - The world into which the assets should be loaded
pub fn load_sprite_sheet<S: Into<String>>(
    asset_name: S,
    asset_conf: S,
    world: &mut World
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(asset_name, ImageFormat::default(), (), &texture_storage)
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        asset_conf,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}
