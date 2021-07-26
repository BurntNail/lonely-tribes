use crate::states::PuzzleState;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::ecs::{Builder, World, WorldExt},
    core::Transform,
    input::{InputEvent, VirtualKeyCode},
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
    {SimpleTrans, StateEvent, Trans},
};
use std::collections::HashMap;

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

pub fn get_trans_puzzle(
    event: StateEvent,
    actions: &HashMap<VirtualKeyCode, usize>,
) -> SimpleTrans {
    let mut t = Trans::None;
    if let StateEvent::Input(event) = event {
        if let InputEvent::KeyPressed { key_code, .. } = event {
            actions.iter().for_each(|(k, v)| {
                if &key_code == k {
                    t = Trans::Switch(Box::new(PuzzleState::new(*v)));
                }
            });
        }
    }
    t
}
