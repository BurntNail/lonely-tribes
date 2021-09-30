use amethyst::{
    core::{
        ecs::{Entities, Entity, Read, System, World, Write, WriteStorage},
        Time,
    },
    ui::{Anchor, FontHandle, LineMode, UiText, UiTransform},
};
use lonely_tribes_animations::{
    animation::Animator, interpolation::AnimInterpolation, tint::TintAnimatorData,
};
use lonely_tribes_lib::states_util::{get_scaling_factor, load_font};
use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Default, Debug)]
pub struct MessageList(pub Vec<String>);

impl Deref for MessageList {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MessageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone)]
pub struct TimedMessagesToAdd {
    pub timer: f32,
    pub list: Vec<(f32, String)>,
}

impl Iterator for TimedMessagesToAdd {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;

        if let Some((t, msg)) = self.list.get(0) {
            if &self.timer > t {
                res = Some(msg.clone());
            }
        }

        if res.is_some() {
            self.list.remove(0);
            self.timer = 0.0;
        }

        res
    }
}

const MESSAGE_PER_LETTER: f32 = 0.1;

#[derive(Default)]
pub struct MessageSystem {
    current: Option<(f32, Entity)>,
    queue: VecDeque<String>,
    font: Option<FontHandle>,
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_UI_TRANS: UiTransform = {
        let (sfx, sfy) = get_scaling_factor();
        UiTransform::new(
            "message_system".to_string(),
            Anchor::TopRight,
            Anchor::TopRight,
            -90.0 * sfx,
            -90.0 * sfy,
            0.5,
            333.3 * sfx,
            500.0 * sfy
        )
    };
}

impl<'s> System<'s> for MessageSystem {
    type SystemData = (
        Entities<'s>,
        Write<'s, MessageList>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, Animator<TintAnimatorData>>,
        Read<'s, Time>,
        Write<'s, TimedMessagesToAdd>,
    );

    fn run(
        &mut self,
        (entites, mut message_list, mut transforms, mut txts, mut aniamtors, time, mut timed_msgs): Self::SystemData,
    ) {
        for msg in std::mem::take(&mut message_list.0) {
            self.queue.push_back(msg);
        }

        let new_ent_needed = if let Some((time_left, ent)) = self.current.borrow_mut() {
            *time_left -= time.delta_real_seconds();
            if time_left > &mut 0.0 {
                false
            } else {
                entites
                    .delete(*ent)
                    .unwrap_or_else(|err| log::warn!("Unable to delete entity: {}", err));
                true
            }
        } else {
            true
        };

        if new_ent_needed {
            self.current = None;

            let msg = if let Some(ms) = self.queue.pop_front() {
                Some(ms)
            } else {
                timed_msgs.timer += time.delta_seconds();
                timed_msgs.next()
            };

            if let Some(msg) = msg {
                if let Some(handle) = self.font.clone() {
                    let time = MESSAGE_PER_LETTER * msg.len() as f32;

                    let (sfx, _sfy) = get_scaling_factor();
                    let txt = UiText::new(
                        handle,
                        msg,
                        [1.0; 4],
                        25.0 * sfx,
                        LineMode::Wrap,
                        Anchor::TopRight,
                    );

                    let anim = Animator::new(TintAnimatorData::new(
                        1.0,
                        0.0,
                        None,
                        time,
                        AnimInterpolation::ReverseExponential,
                    ));

                    let ent = entites
                        .build_entity()
                        .with(DEFAULT_UI_TRANS.clone(), &mut transforms)
                        .with(txt, &mut txts)
                        .with(anim, &mut aniamtors)
                        .build();

                    self.current = Some((time, ent));
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        self.font = Some(load_font(world, "ZxSpectrum"));
    }
}
