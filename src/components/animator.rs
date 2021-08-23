use crate::components::tile_transform::TileTransform;
use amethyst::core::ecs::{Component, DenseVecStorage};
use std::ops::{Deref, DerefMut};

///Type of Interpolation for Animation
#[derive(Copy, Clone, Debug)]
pub enum AnimInterpolation {
    Log10,
    #[allow(dead_code)]
    Linear,
}
impl Default for AnimInterpolation {
    fn default() -> Self {
        Self::Log10
    }
}

///Component to animate a tiletransform horizontally or vertically
#[derive(Copy, Clone, Debug, Default)]
pub struct AnimationData {
    pub start: TileTransform,
    pub end: TileTransform,

    pub total_time: f32,
    pub time_elapsed: f32,

    pub interpolation: AnimInterpolation,
}

impl AnimationData {
    ///Constructor
    pub fn new(start: TileTransform, end: TileTransform, total_time: f32) -> Self {
        Self {
            start,
            end,
            total_time,
            ..Default::default()
        }
    }

    ///Gets x offset
    pub fn x_offset(&self) -> f32 {
        self.get_offset_multiplier() * ((self.start.x - self.end.x) as f32)
    }
    ///Gets y offset
    pub fn y_offset(&self) -> f32 {
        self.get_offset_multiplier() * ((self.start.y - self.end.y) as f32)
    }

    ///Get offset multiplier based on time
    pub fn get_offset_multiplier(&self) -> f32 {
        let base = self.time_elapsed / self.total_time;

        let val = match self.interpolation {
            AnimInterpolation::Log10 => (base * 10.0).log10(),
            AnimInterpolation::Linear => base,
        };

        if val <= 0.0 {
            0.0
        } else {
            val
        }
    }

    ///Whether or not the animator is finished
    pub fn is_done(&self) -> bool {
        self.time_elapsed >= self.total_time
    }
    ///Increments time elapsed
    pub fn add_time(&mut self, time_since_last: f32) {
        self.time_elapsed += time_since_last;
    }
}

///Component on all players to hold an animator
pub struct Animator {
    pub animation_data: Option<AnimationData>,
}
impl Animator {
    ///Constructor which initialises the animationdata as none
    pub fn new() -> Self {
        Self {
            animation_data: None,
        }
    }

    ///Sets the data to none when it is finished
    pub fn finish(&mut self) {
        self.animation_data = None;
    }

    ///Sets the animation data
    pub fn replace_data(&mut self, data: AnimationData) {
        self.animation_data = Some(data);
    }

    ///Whether or not the animation is done - returns false if the current data is none
    pub fn anim_is_done(&self) -> bool {
        if let Some(a) = self.animation_data {
            a.is_done()
        } else {
            false
        }
    }
}

impl Component for Animator {
    type Storage = DenseVecStorage<Self>;
}

impl Deref for Animator {
    type Target = Option<AnimationData>;

    fn deref(&self) -> &Self::Target {
        &self.animation_data
    }
}
impl DerefMut for Animator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animation_data
    }
}
