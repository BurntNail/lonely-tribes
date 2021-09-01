use crate::{components::{
    animator::{AnimationData, Animator, MovementAnimationData},
    tile_transform::TileTransform,
}, HEIGHT, TILE_WIDTH_HEIGHT};
use amethyst::{
    core::{ecs::Read, transform::Transform, Time},
    ecs::{Join, System, WriteStorage},
};

/// System to turn TileTransforms into Transforms
pub struct UpdateTileTransforms;

///Offset x to have tile anchored to centre rather than corner.
pub const TILE_WIDTH: f32 = TILE_WIDTH_HEIGHT as f32 / 2.0;
///Offset y to have tile anchored to centre rather than corner.
pub const TILE_HEIGHT: f32 = TILE_WIDTH_HEIGHT as f32 / 2.0;

impl<'s> System<'s> for UpdateTileTransforms {
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Animator<MovementAnimationData>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (tiles, mut transforms, mut animators, time): Self::SystemData) {
        for (tile, trans) in (&tiles, &mut transforms).join() {
            let old_z = trans.translation().z;
            let x = tile.x as f32 * TILE_WIDTH_HEIGHT as f32 + TILE_WIDTH + tile.x_offset as f32;
            let y = (HEIGHT as f32 - tile.y as f32) * TILE_WIDTH_HEIGHT as f32 - TILE_HEIGHT - tile.y_offset as f32;

            trans.set_translation_xyz(x, y, old_z);
        }

        for (trans, anim_cmp) in (&mut transforms, &mut animators).join() {
            if anim_cmp.anim_is_done() {
                anim_cmp.finish();
            } else if let Some(anim) = &mut anim_cmp.animation_data {
                anim.add_time(time.delta_seconds());

                //Translation
                let start = anim.start;
                let (xo, yo) = anim.get_current();

                let x = ((start.x as f32) - xo) * TILE_WIDTH_HEIGHT as f32 + TILE_WIDTH;
                let y = ((HEIGHT as f32 - start.y as f32) + yo) * TILE_WIDTH_HEIGHT as f32 - TILE_HEIGHT;
                let z = trans.translation().z;

                trans.set_translation_xyz(x, y, z);

                //Rotation
                if (xo != 0.0 || yo != 0.0) && anim.rotates {
                    let rotation = (anim.get_offset_multiplier() * 180.0).to_radians().sin() / 2.0;
                    let mult = if xo > 0.0 || yo > 0.0 { 1.0 } else { -1.0 };
                    trans.set_rotation_2d(mult * rotation);
                } else {
                    trans.set_rotation_2d(0.0);
                }
            }
        }
    }
}

impl UpdateTileTransforms {
    ///Convert a TileTransform to a Transform on Screen
    #[allow(dead_code)]
    pub fn tile_to_transform(tile: TileTransform, z: f32) -> Transform {
        let mut trans = Transform::default();
        let (x, y) = Self::tile_to_xyz(tile);
        trans.set_translation_xyz(x, y, z);
        trans
    }
    ///Convert a TileTransform to an XYZ for a Transform on Screen
    pub fn tile_to_xyz(tile: TileTransform) -> (f32, f32) {
        let x = tile.x as f32 * TILE_WIDTH_HEIGHT as f32 + TILE_WIDTH;
        let y = (HEIGHT - tile.y as u32) as f32 * TILE_WIDTH_HEIGHT as f32 - TILE_HEIGHT;
        (x, y)
    }
}
