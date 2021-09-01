use crate::components::animations::{
    data::AnimationData,
    interpolation::{get_offset_multiplier, AnimInterpolation},
};

#[derive(Copy, Clone, Debug)]
pub struct RotationAnimationData {
    pub total_time: f32,
    pub time_elapsed: f32,

    pub interpolation: AnimInterpolation,
}
impl AnimationData for RotationAnimationData {
    type AnimDataType = f32;

    fn is_done(&self) -> bool {
        self.time_elapsed >= self.total_time
    }

    fn add_time(&mut self, time_since_last: f32) {
        self.time_elapsed += time_since_last;
    }

    fn get_current(&self) -> Self::AnimDataType {

        let om = get_offset_multiplier(self.time_elapsed, self.total_time, self.interpolation);
        let v = (om * 360.0).to_radians().sin() / 2.0;
        log::info!("{}", v);
        v
    }
}

impl RotationAnimationData {
    pub fn new(total_time: f32, interpolation: AnimInterpolation) -> Self {
        Self {
            total_time,
            time_elapsed: 0.0,
            interpolation,
        }
    }
}
