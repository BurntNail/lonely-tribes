use crate::components::animator::{AnimInterpolation, AnimationData};
use amethyst::{
    core::ecs::{Component, DefaultVecStorage, DenseVecStorage},
    renderer::{palette::Srgba, resources::Tint},
};

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub radius: u32,
}
impl PointLight {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}
impl Default for PointLight {
    fn default() -> Self {
        Self { radius: 1 }
    }
}
impl Component for PointLight {
    type Storage = DefaultVecStorage<Self>;
}

#[derive(Copy, Clone, Debug)]
pub struct TintOverride(pub Tint);
impl Component for TintOverride {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone, Debug)]
pub struct TintAnimatorData {
    pub start: f32,
    pub end: f32,
    pub override_tint: Option<Tint>,

    pub total_time: f32,
    pub time_elapsed: f32,

    pub interpolation: AnimInterpolation,
}
impl TintAnimatorData {
    pub fn new(
        start: f32,
        end: f32,
        override_tint: Option<Tint>,
        total_time: f32,
        interpolation: AnimInterpolation,
    ) -> Self {
        TintAnimatorData {
            start,
            end,
            override_tint,
            total_time,
            time_elapsed: 0.0,
            interpolation,
        }
    }
}

impl AnimationData for TintAnimatorData {
    type AnimDataType = Tint;

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
        self.time_elapsed += time_since_last
    }

    fn get_current(&self) -> Self::AnimDataType {
        let factor = self.start + (self.end - self.start) * self.get_offset_multiplier();
        let factor: f32 = {
            let str_version = format!("{:03}", factor);
            str_version.parse().unwrap_or_else(|err| {
                log::warn!("Couldn't parse into str because: {}", err);
                1.0
            })
        };

        let or = self.override_tint.unwrap_or(Tint(Srgba::new(1.0, 1.0, 1.0, 1.0)));
        Tint(Srgba::new(or.0.red * factor, or.0.green * factor, or.0.blue * factor, or.0.alpha * factor))
    }
}
