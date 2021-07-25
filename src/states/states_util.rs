use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::ui::{FontAsset, TtfFormat};

pub fn init_camera(world: &mut World, wh: (f32, f32)) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(wh.0 * 0.5, wh.1 * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(wh.0, wh.1))
        .with(transform)
        .build();
}

pub fn load_font(world: &mut World, path: &str) -> Handle<FontAsset> {
    world.read_resource::<Loader>().load(
        format!("fonts/{}.ttf", path),
        TtfFormat,
        (),
        &world.read_resource(),
    )
}

pub fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    log::info!("Loading sprite sheet: {}", path);
    let tex_handle = world.read_resource::<Loader>().load(
        format!("{}.png", path),
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    world.read_resource::<Loader>().load(
        format!("{}.ron", path),
        SpriteSheetFormat(tex_handle),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}
