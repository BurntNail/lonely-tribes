use crate::{
    components::{Animator, TileTransform},
    HEIGHT,
};
use amethyst::{
    core::{
        ecs::{Entities, Read},
        transform::Transform,
        Time,
    },
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct UpdateTileTransforms;

pub const TILE_WIDTH: f32 = 8.0 / 2.0;
pub const TILE_HEIGHT: f32 = 8.0 / 2.0;

impl<'s> System<'s> for UpdateTileTransforms {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Animator>,
        Read<'s, Time>,
    );

    fn run(&mut self, (tiles, mut transforms, mut animators, time): Self::SystemData) {
        for (tile, trans) in (&tiles, &mut transforms).join() {
            let old_z = trans.translation().z;
            let x = tile.x as f32 * 8.0 + TILE_WIDTH;
            let y = (HEIGHT - tile.y as u32) as f32 * 8.0 - TILE_HEIGHT;

            trans.set_translation_xyz(x, y, old_z);
        }

        for (tile, trans, anim) in (&tiles, &mut transforms, &mut animators).join() {
            let old_z = trans.translation().z;
            let mut x = tile.x as f32 * 8.0 + TILE_WIDTH;
            let mut y = (HEIGHT - tile.y as u32) as f32 * 8.0 - TILE_HEIGHT;

            anim.time_elapsed += time.delta_real_seconds();
            let point_reached = anim.total_time / anim.time_elapsed;

            let addition = ((anim.difference.x as f32) * point_reached, (anim.difference.y as f32) * point_reached);
            let addition = Self::tile_xy_f32s_to_xy(addition);


            trans.set_translation_xyz(x + addition.0, y + addition.1, old_z);
        }
    }
}

impl UpdateTileTransforms {
    ///Convert a TileTransform to a Transform on Screen
    pub fn tile_to_transform(tile: TileTransform) -> Transform {
        let mut trans = Transform::default();
        let (x, y, z) = Self::tile_to_xyz(tile);
        trans.set_translation_xyz(x, y, z);
        trans
    }
    ///Convert a TileTransform to an XYZ for a Transform on Screen
    pub fn tile_to_xyz(tile: TileTransform) -> (f32, f32, f32) {
        let z = 0.0;
        let x = tile.x as f32 * 8.0 + TILE_WIDTH;
        let y = (HEIGHT - tile.y as u32) as f32 * 8.0 - TILE_HEIGHT;
        (x, y, z)
    }
    ///Convert a TileTransform X and Y (but as floats in a tuple) to an X and a Y for an on-screen Transform
    pub fn tile_xy_f32s_to_xy(xy: (f32, f32)) -> (f32, f32) {
        let x = xy.0 * 8.0 + TILE_WIDTH;
        let y = (HEIGHT as f32 - xy.1) * 8.0 - TILE_HEIGHT;
        (x, y)
    }
}
