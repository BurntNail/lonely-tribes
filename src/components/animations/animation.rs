use crate::components::animations::data::AnimationData;
use amethyst::core::ecs::{Component, DenseVecStorage};
use std::ops::{Deref, DerefMut};

///Component on all players to hold an animations
#[derive(Copy, Clone, Debug, Default)]
pub struct Animator<T: AnimationData> {
    pub animation_data: Option<T>,
}
impl<T: AnimationData> Animator<T> {
    ///Constructor which initialises the animationdata as none
    pub fn new() -> Self {
        Self {
            animation_data: None,
        }
    }

    ///Sets the data to none when it is finished
    pub fn finish(&mut self) {
        if self.anim_is_done() {
            self.animation_data = None;
        }
    }

    ///Sets the animation data
    pub fn replace_data(&mut self, data: T) {
        self.animation_data = Some(data);
    }

    ///Whether or not the animation is done - returns false if the current data is none
    pub fn anim_is_done(&self) -> bool {
        if let Some(a) = &self.animation_data {
            a.is_done()
        } else {
            false
        }
    }

    pub fn add_time(&mut self, t: f32) {
        if let Some(a) = &mut self.animation_data {
            a.add_time(t);
        }
    }
}

impl<T: 'static + AnimationData + Sync + Send> Component for Animator<T> {
    type Storage = DenseVecStorage<Self>;
}

impl<T: AnimationData> Deref for Animator<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.animation_data
    }
}
impl<T: AnimationData> DerefMut for Animator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animation_data
    }
}
