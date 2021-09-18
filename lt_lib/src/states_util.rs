use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        ecs::{Builder, World, WorldExt},
        Transform,
    },
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
};
use std::{fs::read_dir, path::Path};

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
    c.conf.screen_dimensions.0 as f32 / 1600.0 //game was originally designed for 1600x900
}

pub fn get_levels() -> Vec<(String, bool)> {
    let mut out: Vec<(String, bool)> = list_file_names_in_dir("assets/maps")
        .into_iter()
        .filter_map(|nom| {
            let is_normal = nom.contains("lvl-") && nom.contains(".png");
            let is_pg = nom.contains("pg-") && nom.contains(".txt");
            let name = nom.replace("\"", "");

            if is_normal || is_pg {
                Some((name, is_normal))
            } else {
                None
            }
        })
        .collect();
    out.sort();
    out
}
pub fn get_levels_str() -> Vec<String> {
    get_levels().into_iter().map(|(s, _)| s).collect()
}
pub fn levels_len() -> usize {
    if let Ok(read) = read_dir("assets/maps") {
        read.count()
    } else {
        0
    }
}

///Gets file names inside a directory
pub fn list_file_names_in_dir<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut list = Vec::new();
    if let Ok(read) = read_dir(path) {
        read.for_each(|el| {
            if let Ok(el) = el {
                let current_file = format!("{:?}", el.file_name());
                list.push(current_file);
            }
        });

        list.sort();
        list.reverse();
    }

    list
}
