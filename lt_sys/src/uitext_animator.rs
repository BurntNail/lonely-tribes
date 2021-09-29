use amethyst::core::ecs::{System, WriteStorage, Join};
use lonely_tribes_animations::animation::Animator;
use lonely_tribes_animations::tint::TintAnimatorData;
use amethyst::ui::UiText;

pub struct UiTextAnimator;

impl<'s> System<'s> for UiTextAnimator {
	type SystemData = (
		WriteStorage<'s, Animator<TintAnimatorData>>,
		WriteStorage<'s, UiText>
	);

	fn run(&mut self, (mut animators, mut txts): Self::SystemData) {
		for (anim, txt) in (&mut animators, &mut txts).join() {
			txt.color = [1.0, 1.0, 1.0, anim.]
		}
	}
}