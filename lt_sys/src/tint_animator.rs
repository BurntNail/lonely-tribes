use amethyst::{
    core::{
        ecs::{Join, Read, System, WriteStorage},
        Time,
    },
    renderer::resources::Tint,
};
use lonely_tribes_animations::{animation::Animator, data::AnimationData, tint::TintAnimatorData};

pub struct TintAnimatorSystem;

impl<'s> System<'s> for TintAnimatorSystem {
    type SystemData = (
        WriteStorage<'s, Animator<TintAnimatorData>>,
        WriteStorage<'s, Tint>,
        Read<'s, Time>,
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
                } else if tint.0.alpha > 0.999 {
                    tint.0.alpha = 1.0;
                    anim.finish();
                }
            }
        }
    }
}
