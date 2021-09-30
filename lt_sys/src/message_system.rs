use amethyst::{
    core::{
        ecs::{Entity, Read, System, World, Write, WriteStorage},
        Time,
    },
    ui::{Anchor, FontHandle, LineMode, UiText, UiTransform},
};
use lonely_tribes_lib::states_util::{get_scaling_factor, load_font};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};
use amethyst::core::ecs::Entities;
use std::borrow::BorrowMut;
use lonely_tribes_animations::animation::Animator;
use lonely_tribes_animations::tint::TintAnimatorData;
use lonely_tribes_animations::interpolation::AnimInterpolation;

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

const MESSAGE_TOTAL_TIME: f32 = 1.0;

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
            100.0 * sfx,
            100.0 * sfy
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
    );

    fn run(
        &mut self,
        (entites, mut message_list, mut transforms, mut txts, mut aniamtors, time): Self::SystemData,
    ) {
        for msg in std::mem::take(&mut message_list.0) {
            log::info!("{}", &msg);
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

            if let Some(msg) = self.queue.pop_front() {
                if let Some(handle) = self.font.clone() {
                    let (sfx, _sfy) = get_scaling_factor();
                    let txt = UiText::new(
                        handle,
                        msg,
                        [1.0; 4],
                        25.0 * sfx,
                        LineMode::Wrap,
                        Anchor::TopRight,
                    );
                    let anim = Animator::new(
                        TintAnimatorData::new(1.0, 0.0, None, MESSAGE_TOTAL_TIME, AnimInterpolation::ReverseExponential)
                    ); //TODO: Fix by writing a new animator system for uitext objs

                    let ent = entites
                        .build_entity()
                        .with(DEFAULT_UI_TRANS.clone(), &mut transforms)
                        .with(txt, &mut txts)
                        .with(anim, &mut aniamtors)
                        .build();

                    self.current = Some((MESSAGE_TOTAL_TIME, ent));
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        self.font = Some(load_font(world, "ZxSpectrum"));
    }
}
