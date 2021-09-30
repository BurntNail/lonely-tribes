use amethyst::core::ecs::{System, WriteStorage, Join, Read};
use lonely_tribes_animations::animation::Animator;
use lonely_tribes_animations::tint::TintAnimatorData;
use amethyst::ui::UiText;
use amethyst::core::Time;
use lonely_tribes_animations::data::AnimationData;

pub struct UiTextAnimator;

impl<'s> System<'s> for UiTextAnimator {
	type SystemData = (
		WriteStorage<'s, Animator<TintAnimatorData>>,
		WriteStorage<'s, UiText>,
		Read<'s, Time>
	);

	fn run(&mut self, (mut animators, mut txts, time): Self::SystemData) {
		for (anim, txt) in (&mut animators, &mut txts).join() {
			anim.add_time(time.delta_seconds());
			
			if let Some(data) = &mut anim.animation_data {
				if data.is_done() {
					anim.finish();
					continue;
				}
				
				txt.color = [1.0, 1.0, 1.0, data.get_current().0.alpha];
			}
		}
	}
}