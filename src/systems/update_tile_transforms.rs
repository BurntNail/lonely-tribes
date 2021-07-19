use amethyst::core::math::Vector3;
use amethyst::{
    core::{transform::Transform, Time},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::Camera,
};
use crate::level::Room;
use crate::components::TileTransform;
use crate::{WIDTH, HEIGHT};

#[derive(SystemDesc)]
pub struct UpdateTileTransforms;

const TILE_WIDTH: f32 = 8.0 / 2.0;
const TILE_HEIGHT: f32 = 8.0 / 2.0;

impl<'s> System<'s> for UpdateTileTransforms {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (tiles, mut transforms): Self::SystemData) {
        for (tile, trans) in (&tiles, &mut transforms).join() {
            let old_z = trans.translation().z;
            trans.set_translation_xyz(
                tile.x as f32 * 8.0 + TILE_WIDTH,
                (HEIGHT as usize - tile.y) as f32 * 8.0 + TILE_HEIGHT,
                old_z
            );
        }
    }
}
