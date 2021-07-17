use amethyst::{
    assets::{AssetStorage},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use log::info;
use amethyst::assets::{Loader, Handle};
use amethyst::tiles::{TileMap, MapStorage};
use crate::tiles::SimpleTile;
use amethyst::core::math::{Vector3, Point3};
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::SpriteRender;

pub struct MyState;

pub const WIDTH: u32 = 8 * 16 * 2;
pub const HEIGHT: u32 = 8 * 9 * 2; //each sprite is 8px wide


impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = ScreenDimensions::new(WIDTH, HEIGHT, 1.0); //No idea what HDPI is, so have set it to 1
        init_camera(world, &dimensions);

        let sprite_handle = load_sprite_sheet(world, "art/colored_tilemap_packed");
        // let map = TileMap::<SimpleTile>::new(
        //     Vector3::new(8, 8, 1), // The dimensions of the map
        //     Vector3::new(14, 10, 1), // The dimensions of each tile
        //     Some(sprite_handle),
        // );

        let mut t = Transform::default();
        t.set_translation_xyz(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0);

        let spr = SpriteRender::new(sprite_handle, 5);

        world
            .create_entity()
            .with(spr)
            .with(t)
            .build();
    }
}

fn load_sprite_sheet (world: &mut World, path: &str) -> Handle<SpriteSheet> {
    info!("Loaded sprite sheet: {}", path);
    let tex_handle =
        world.read_resource::<Loader>().load(format!("{}.png", path), ImageFormat::default(), (), &world.read_resource::<AssetStorage<Texture>>());
    world.read_resource::<Loader>().load(
        format!("{}.ron", path),
        SpriteSheetFormat(tex_handle),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>()
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
