use crate::{
    data::AnimationData,
    interpolation::{get_offset_multiplier, AnimInterpolation},
};
use amethyst::renderer::{palette::Srgba, resources::Tint};

#[derive(Copy, Clone, Debug)]
///component to change an entitiy's tint
pub struct TintAnimatorData {
    ///Start rotation
    pub start: f32,
    ///End rotation
    pub end: f32,
    ///Override tint for if the object has an active tint
    pub override_tint: Option<Tint>,

    ///Total animation length
    pub total_time: f32,
    ///Time elapsed so far for the animation
    pub time_elapsed: f32,

    ///Interpolation type for the rotation
    pub interpolation: AnimInterpolation,
}
impl TintAnimatorData {
    ///Constructor
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

    fn is_done(&self) -> bool {
        self.time_elapsed >= self.total_time
    }

    fn add_time(&mut self, time_since_last: f32) {
        self.time_elapsed += time_since_last
    }

    fn get_current(&self) -> Self::AnimDataType {
        let factor = {
            let f = self.start
                + (self.end - self.start)
                    * get_offset_multiplier(self.time_elapsed, self.total_time, self.interpolation);
            let str_version = format!("{:03}", f);
            str_version.parse().unwrap_or_else(|err| {
                log::warn!("Couldn't parse into str because: {}", err);
                1.0
            })
        };

        if let Some(or) = self.override_tint {
            Tint(Srgba::new(or.0.red, or.0.green, or.0.blue, factor))
        } else {
            Tint(Srgba::new(factor, factor, factor, factor))
        }
    }
}
