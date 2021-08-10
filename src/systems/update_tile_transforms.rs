use crate::{
    components::{animator::Animator, tile_transform::TileTransform},
    HEIGHT,
};
use amethyst::{
    core::{ecs::Read, transform::Transform, Time},
    ecs::{Join, System, WriteStorage},
};

/// System to turn TileTransforms into Transforms
pub struct UpdateTileTransforms;

///Offset x to have tile anchored to centre rather than corner.
pub const TILE_WIDTH: f32 = 8.0 / 2.0;
///Offset y to have tile anchored to centre rather than corner.
pub const TILE_HEIGHT: f32 = 8.0 / 2.0;

impl<'s> System<'s> for UpdateTileTransforms {
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Animator>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut tiles, mut transforms, mut animators, time): Self::SystemData) {
        for (tile, trans) in (&tiles, &mut transforms).join() {
            let old_z = trans.translation().z;
            let x = tile.x as f32 * 8.0 + TILE_WIDTH;
            let y = (HEIGHT - tile.y as u32) as f32 * 8.0 - TILE_HEIGHT;

            trans.set_translation_xyz(x, y, old_z);
        }

        for (trans, anim_cmp, tt) in (&mut transforms, &mut animators, &mut tiles).join() {
            let z = trans.translation().z;
            let mut x = tt.x as f32 * 8.0 + TILE_WIDTH;
            let mut y = (HEIGHT - tt.y as u32) as f32 * 8.0 - TILE_HEIGHT;

            if anim_cmp.anim_is_done() {
                anim_cmp.finish();
            } else if let Some(anim) = &mut anim_cmp.animation_data {
                let end = anim.end;

                x = ((end.x as f32) - anim.x_offset()) * 8.0 + TILE_WIDTH;
                y = ((HEIGHT as f32 - end.y as f32) - anim.y_offset()) * 8.0 - TILE_HEIGHT;

                anim.add_time(time.delta_seconds());
            }

            trans.set_translation_xyz(x, y, z);
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
    #[allow(dead_code)]
    pub fn tile_xy_f32s_to_xy(xy: (f32, f32)) -> (f32, f32) {
        let x = xy.0 * 8.0 + TILE_WIDTH;
        let y = (HEIGHT as f32 - xy.1) * 8.0 - TILE_HEIGHT;
        (x, y)
    }
}
