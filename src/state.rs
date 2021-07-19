use amethyst::{
    assets::AssetStorage,
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use crate::level::Room;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::assets::{Handle, Loader};
use log::info;
use amethyst::renderer::SpriteRender;
use crate::components::TileTransform;

#[derive(Default)]
pub struct MyState {
    handle: Option<Handle<SpriteSheet>>,
    app_root_dir: String
}

impl MyState {
    pub fn new (app_root_dir: String) -> Self {
        Self {
            handle: None,
            app_root_dir
        }
    }
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = ScreenDimensions::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0); //No idea what HDPI is, so have set it to 1
        init_camera(world, &dimensions);

        self.handle
            .replace(load_sprite_sheet(world, "art/colored_tilemap_packed"));

        // let mut lvl_path = self.app_root_dir.clone();
        // lvl_path.push_str("/maps/test.ron");
        let lvl_path = "assets/maps/test.ron".to_string(); //TODO: Fix FQDN
        load_level(world, self.handle.clone().unwrap(), lvl_path.as_str());
    }
}

fn load_level(world: &mut World, sprites_handle: Handle<SpriteSheet>, path: &str) {
    let lvl = Room::new(path);

    if lvl.data.is_empty() {
        return;
    }

    for x in 0..lvl.data.len() {
        for y in 0..lvl.data[0].len() {
            let spr_index = lvl.data[x][y].get_index();

            if spr_index == 16 {
                continue;
            }

            let spr = SpriteRender::new(sprites_handle.clone(), spr_index);

            world
                .create_entity()
                .with(spr)
                .with(Transform::default())
                .with(TileTransform::new(x, y))
                .build();
        }
    }
}

fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    info!("Loading sprite sheet: {}", path);
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

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}