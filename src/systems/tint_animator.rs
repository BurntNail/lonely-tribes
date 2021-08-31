use amethyst::core::ecs::{System, WriteStorage, Read, Join};
use crate::components::animator::{Animator, AnimationData};
use crate::components::point_light::TintAnimatorData;
use amethyst::renderer::resources::Tint;
use amethyst::core::Time;

pub struct TintAnimatorSystem;

impl <'s> System<'s> for TintAnimatorSystem {
    type SystemData = (
        WriteStorage<'s, Animator<TintAnimatorData>>,
        WriteStorage<'s, Tint>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut animators, mut tints, time): Self::SystemData) {
        for (anim, tint) in (&mut animators, &mut tints).join() {
            anim.add_time(time.delta_seconds());

            if let Some(d) = &mut anim.animation_data {
                if d.is_done() {
                    anim.finish();
                    continue;
                }

                let new_tint: Tint = d.get_current();
                tint.0 = new_tint.0;
                if tint.0.alpha < 0.001 {
                    tint.0.alpha = 0.0;
                    anim.finish();
                }
            }
        }
    }
}