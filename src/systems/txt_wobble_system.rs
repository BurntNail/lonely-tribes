use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Read, Join};
use crate::components::TextWobble;
use amethyst::ui::UiTransform;
use amethyst::core::Time;

pub struct TextWobbleSystem;

impl <'s> System<'s> for TextWobbleSystem {
    type SystemData = (
        WriteStorage<'s, TextWobble>,
        WriteStorage<'s, UiTransform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut txt_wobbles, mut ui_transforms, time): Self::SystemData) {
        for (wobble, trans) in (&mut txt_wobbles, &mut ui_transforms).join() {
            let degrees = (wobble.current_time / wobble.duration) * 360.0;
            let cos_val = degrees.to_radians().cos();

            wobble.current_time += time.delta_seconds();

            let offset = cos_val * wobble.distance / 2.0;
            trans.local_y = wobble.old_y + offset;

        }
    }
}
