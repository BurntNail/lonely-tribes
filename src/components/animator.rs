use crate::components::tile_transform::TileTransform;
use amethyst::core::ecs::{Component, DenseVecStorage};
use std::ops::{Deref, DerefMut};

///Type of Interpolation for Animation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnimInterpolation {
    ReverseExponential,
    #[allow(dead_code)]
    Linear,
}

const A: f32 = 2.0;
const B: f32 = -6.5;

impl Default for AnimInterpolation {
    fn default() -> Self {
        Self::ReverseExponential
    }
}
impl AnimInterpolation {
    pub fn get_val_from_pctg(&self, pctg: f32) -> f32 {
        match self {
            AnimInterpolation::ReverseExponential => {
                //worked out with desmos
                //https://www.desmos.com/calculator/sbq8tbhr9d
                (-1.0 * A.powf(B * pctg)) + 1.0
            }
            AnimInterpolation::Linear => pctg,
        }
    }
}

pub trait AnimationData {
    type AnimDataType;

    fn get_offset_multiplier(&self) -> f32;
    fn is_done(&self) -> bool;
    fn add_time(&mut self, time_since_last: f32);
    fn get_current(&self) -> Self::AnimDataType;
}

///Component to animate a tiletransform horizontally or vertically
#[derive(Copy, Clone, Debug)]
pub struct MovementAnimationData {
    pub start: TileTransform,
    pub end: TileTransform,

    pub total_time: f32,
    pub time_elapsed: f32,

    pub interpolation: AnimInterpolation,
    pub rotates: bool,
}
impl Default for MovementAnimationData {
    fn default() -> Self {
        Self {
            rotates: true,

            start: TileTransform::default(),
            end: TileTransform::default(),
            total_time: 0.0,
            time_elapsed: 0.0,
            interpolation: AnimInterpolation::default(),
        }
    }
}

impl MovementAnimationData {
    ///Constructor
    pub fn new(
        start: TileTransform,
        end: TileTransform,
        total_time: f32,
        interp: AnimInterpolation,
    ) -> Self {
        Self {
            start,
            end,
            total_time,
            interpolation: interp,
            ..Default::default()
        }
    }
    pub fn new_no_rotate(
        start: TileTransform,
        end: TileTransform,
        total_time: f32,
        interp: AnimInterpolation,
    ) -> Self {
        Self {
            start,
            end,
            total_time,
            rotates: false,
            interpolation: interp,
            ..Default::default()
        }
    }
}

impl AnimationData for MovementAnimationData {
    type AnimDataType = (f32, f32);

    fn get_offset_multiplier(&self) -> f32 {
        let base = self.time_elapsed / self.total_time;

        let val: f32 = {
            let str_version = format!("{:03}", self.interpolation.get_val_from_pctg(base)); //get down to 3dp
            str_version.parse().unwrap_or_else(|err| {
                log::warn!("Couldn't parse into str because: {}", err);
                1.0
            })
        };

        if val <= 0.0 {
            0.0
        } else {
            val
        }
    }

    fn is_done(&self) -> bool {
        self.time_elapsed >= self.total_time
    }

    fn add_time(&mut self, time_since_last: f32) {
        self.time_elapsed += time_since_last;
    }

    fn get_current(&self) -> Self::AnimDataType {
        let om = self.get_offset_multiplier();
        let x = om * ((self.start.x - self.end.x) as f32);
        let y = om * ((self.start.y - self.end.y) as f32);
        (x, y)
    }
}

///Component on all players to hold an animator
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

    pub fn add_time (&mut self, t: f32) {
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

#[cfg(test)]
mod anim_tests {
    use super::*;

    ///returns a 1 second linear animatiom from (0,0) to (1,1)
    fn get_anim() -> Animator<MovementAnimationData> {
        let mut a = Animator::new();
        a.replace_data(MovementAnimationData::new(
            (0, 0).into(),
            (0, 1).into(),
            1.0,
            AnimInterpolation::Linear,
        ));
        a
    }

    #[test]
    pub fn new_animator_test() {
        let mut t = Animator::new();
        assert!(t.animation_data.is_none());

        let data = MovementAnimationData::new(
            (0, 0).into(),
            (0, 1).into(),
            1.0,
            AnimInterpolation::Linear,
        );
        t.replace_data(data);
        assert!(t.animation_data.is_some());
        assert_eq!(
            t.animation_data.unwrap().interpolation,
            AnimInterpolation::Linear
        );
    }

    #[test]
    pub fn timing_test() {
        let mut a = get_anim();

        let mut t = 0.0;
        loop {
            if t > 0.9 {
                break;
            }
            t += 0.1;

            if let Some(a) = &mut a.animation_data {
                a.add_time(0.1);

                assert!((a.get_offset_multiplier() - t).abs() <= f32::EPSILON);
            } else {
                assert!(false)
            }
        }
        assert!(!a.anim_is_done());

        if let Some(a) = &mut a.animation_data {
            a.add_time(0.1);
        } else {
            assert!(false);
        }

        assert!(a.anim_is_done());
    }

    #[test]
    pub fn ending_test_large() {
        let mut a = get_anim();

        if let Some(a) = &mut a.animation_data {
            a.add_time(1000.0);
        } else {
            assert!(false);
        }

        a.finish();
        assert!(a.is_none());
    }

    #[test]
    pub fn ending_test_small() {
        let mut a = get_anim();

        if let Some(a) = &mut a.animation_data {
            a.add_time(0.1);
        } else {
            assert!(false);
        }

        a.finish();
        assert!(a.is_some());
    }
}
