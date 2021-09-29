use crate::data::AnimationData;
use amethyst::core::ecs::{Component, DenseVecStorage};
use std::ops::{Deref, DerefMut};

///Component on all players to hold an lt_animations
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Animator<T: AnimationData + Sync + Send + 'static + Copy> {
    pub animation_data: Option<T>,
}
impl<T: AnimationData + Sync + Send + 'static + Copy> Animator<T> {
    #[allow(dead_code)]
    pub fn new(animation_data: T) -> Self {
        Self {
            animation_data: Some(animation_data),
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
            true
        }
    }

    ///Increases self.time_elapsed if it has an animation currently in
    pub fn add_time(&mut self, t: f32) {
        if let Some(a) = &mut self.animation_data {
            a.add_time(t);
        }
    }
}

impl<T: 'static + AnimationData + Sync + Send + Copy> Default for Animator<T> {
    fn default() -> Self {
        Self {
            animation_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movement::MovementAnimationData;

    #[test]
    pub fn default_test() {
        assert_eq!(
            Animator::<MovementAnimationData>::default().animation_data,
            None
        );
    }

    #[test]
    pub fn no_anim_set_test() {
        let a = Animator::<MovementAnimationData>::default();
        assert!(a.anim_is_done());

        let mut b = a.clone();
        b.add_time(1.0);
        assert_eq!(a, b);
    }
}

impl<T: 'static + AnimationData + Sync + Send + Copy> Component for Animator<T> {
    type Storage = DenseVecStorage<Self>;
}

impl<T: 'static + AnimationData + Sync + Send + Copy> Deref for Animator<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.animation_data
    }
}
impl<T: 'static + AnimationData + Sync + Send + Copy> DerefMut for Animator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animation_data
    }
}
