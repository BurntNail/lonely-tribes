use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        ecs::{Builder, World, WorldExt},
        Transform,
    },
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
};

///Helper function to initialise a camera in the world
///
///  - **world** is the Specs World
///  - **wh** is a tuple containing the width and the height in f32s
pub fn init_camera(world: &mut World, wh: (f32, f32)) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(wh.0 * 0.5, wh.1 * 0.5, 100.0);

    world
        .create_entity()
        .with(Camera::standard_2d(wh.0, wh.1))
        .with(transform)
        .build();
}

///Helper function to load in a font, given the world, and a path (eg. *ZxSpectrum* (no ttf or path required))
pub fn load_font(world: &mut World, name: &str) -> Handle<FontAsset> {
    world.read_resource::<Loader>().load(
        format!("fonts/{}.ttf", name),
        TtfFormat,
        (),
        &world.read_resource(),
    )
}

///Helper function to load in a font, given the world, and a path (without an extension, eg. *SpriteSheetPacked*)
///
/// The function assumes that there is a ron file with details of the spritesheet
///
/// Returns a handle to a spritesheet
pub fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let tex_handle = world.read_resource::<Loader>().load(
        format!("art/{}.png", path),
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    world.read_resource::<Loader>().load(
        format!("art/{}.ron", path),
        SpriteSheetFormat(tex_handle),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

pub fn get_scaling_factor() -> f32 {
    let c = crate::config::LTConfig::new();
    let s = c.conf.screen_dimensions.0 as f32 / 1600.0; //game was originally designed for 1600x900
    s
}
