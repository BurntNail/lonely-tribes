use crate::move_player::HELD_INTERVAL;
use amethyst::{
    core::ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::resources::Tint,
};
use lonely_tribes_animations::{
    animation::Animator, interpolation::AnimInterpolation, tint::TintAnimatorData,
};
use lonely_tribes_components::{
    colliders::ColliderList,
    point_light::{PointLight, TintOverride},
    tile_transform::TileTransform,
};
use lonely_tribes_fog_of_war::{fog::LightCacher, light_list::LightList};

#[derive(Default)]
pub struct FogOfWarSystem {
    cacher: LightCacher,
}

pub const TINT_ANIMATION_TIME: f32 = HELD_INTERVAL;

impl<'s> System<'s> for FogOfWarSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, Tint>,
        Read<'s, LightList>,
        Read<'s, ColliderList>,
        ReadStorage<'s, TintOverride>,
        WriteStorage<'s, Animator<TintAnimatorData>>,
    );

    fn run(
        &mut self,
        (tiles, tints, lights, collider_list, overrides, mut animators): Self::SystemData,
    ) {
        let lighted_cells = self
            .cacher
            .get_lighted_cells(lights.get(), collider_list.get());

        for (tile, tint, anim) in (&tiles, &tints, &mut animators).join() {
            let factor = *lighted_cells.get(tile).unwrap_or(&0.0);
            anim.replace_data(TintAnimatorData::new(
                tint.0.alpha,
                factor,
                None,
                TINT_ANIMATION_TIME,
                AnimInterpolation::Linear,
            ));
        }

        for (tile, tint, t_override, anim) in (&tiles, &tints, &overrides, &mut animators).join() {
            let factor = *lighted_cells.get(tile).unwrap_or(&0.0);
            anim.replace_data(TintAnimatorData::new(
                tint.0.alpha,
                factor,
                Some(t_override.0),
                TINT_ANIMATION_TIME,
                AnimInterpolation::Linear,
            ));
        }
    }
}

pub struct LightListSystem;

impl<'s> System<'s> for LightListSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, PointLight>,
        Write<'s, LightList>,
    );

    fn run(&mut self, (tiles, lights, mut light_list): Self::SystemData) {
        let mut list = Vec::new();
        for (t, p) in (&tiles, &lights).join() {
            list.push((*t, *p));
        }
        light_list.set(list);
    }
}
