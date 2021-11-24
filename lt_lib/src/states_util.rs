use crate::{paths::get_directory, CONFIG, HEIGHT, TILE_WIDTH_HEIGHT, WIDTH};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        ecs::{Builder, World, WorldExt},
        Transform,
    },
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
};
use lazy_static::lazy_static;
use std::{cmp::Ordering, fs::read_dir, path::Path};

pub const CAMERA_BASE_WIDTH: f32 = (TILE_WIDTH_HEIGHT * WIDTH) as f32; //For ingame-transform Measurements
pub const CAMERA_BASE_HEIGHT: f32 = (TILE_WIDTH_HEIGHT * HEIGHT) as f32;

lazy_static! {
    pub static ref CAMERA_WIDTH_MULTIPLIER: (f32, f32) = {
        let (w, h) = CONFIG.conf.screen_dimensions;
        let (x, y) = (w as f32 / CAMERA_BASE_WIDTH, h as f32 / CAMERA_BASE_HEIGHT);

        if x > y {
            (1.0, y / x)
        } else {
            (x / y, 1.0)
        }
    };
    pub static ref CAMERA_DIMENSIONS: (f32, f32) = {
        let (x, y) = *CAMERA_WIDTH_MULTIPLIER;
        (CAMERA_BASE_WIDTH * x, CAMERA_BASE_HEIGHT * y)
    };
}

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

pub fn get_scaling_factor_non_normalised() -> (f32, f32) {
    let c = CONFIG.conf;
    (
        c.screen_dimensions.0 as f32 / 1600.0, //game was originally designed for 1600x900
        c.screen_dimensions.1 as f32 / 900.0,
    )
}
pub fn get_scaling_factor() -> (f32, f32) {
    let (x, y) = get_scaling_factor_non_normalised();

    if x > y {
        (1.0, y / x)
    } else {
        (x / y, 1.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LevelType {
    Developer,
    ProcGen,
}
impl LevelType {
    pub(crate) fn id(&self) -> u8 {
        match self {
            LevelType::Developer => 0,
            LevelType::ProcGen => 2,
        }
    }
}

impl PartialOrd<Self> for LevelType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id().partial_cmp(&other.id())
    }
}
impl Ord for LevelType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

pub fn get_levels() -> Vec<(String, LevelType)> {
    let mut out: Vec<(String, LevelType)> = list_file_names_in_dir("../maps", false)
        .into_iter()
        .filter_map(|nom| {
            let mut t = None;
            if nom.contains("lvl-") && nom.contains(".ron") {
                t = Some(LevelType::Developer);
            } else if nom.contains("pg-") && nom.contains(".ron") {
                t = Some(LevelType::ProcGen);
            }

            let name = nom.replace("\"", "");

            t.map(|t| (name, t))
        })
        .collect();
    out.sort();
    log::info!("Got: {:?}", out);
    out
}
pub fn get_levels_str() -> Vec<String> {
    get_levels().into_iter().map(|(s, _)| s).collect()
}
pub fn levels_len() -> usize {
    if let Ok(read) = read_dir(get_directory(false).join("../maps/")) {
        read.count()
    } else {
        0
    }
}

///Gets file names inside a directory
pub fn list_file_names_in_dir<P: AsRef<Path>>(path: P, is_config: bool) -> Vec<String> {
    let mut list = Vec::new();
    if let Ok(read) = read_dir(get_directory(is_config).join(path)) {
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
