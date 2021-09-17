use crate::interpolation::{AnimInterpolation, get_offset_multiplier};
use lonely_tribes_components::tile_transform::TileTransform;
use crate::data::AnimationData;

///Component to animate a tiletransform horizontally or vertically
#[derive(Copy, Clone, Debug)]
pub struct MovementAnimationData {
    ///start position
    pub start: TileTransform,
    ///End Position
    pub end: TileTransform,

    ///Animation Length
    pub total_time: f32,
    ///Time elapsed in animation so far
    pub time_elapsed: f32,

    ///interpolation type
    pub interpolation: AnimInterpolation,
}
impl Default for MovementAnimationData {
    fn default() -> Self {
        Self {
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
}

impl AnimationData for MovementAnimationData {
    type AnimDataType = (f32, f32);

    fn is_done(&self) -> bool {
        self.time_elapsed >= self.total_time
    }

    fn add_time(&mut self, time_since_last: f32) {
        self.time_elapsed += time_since_last;
    }

    fn get_current(&self) -> Self::AnimDataType {
        let om = get_offset_multiplier(self.time_elapsed, self.total_time, self.interpolation);
        let x = om * ((self.start.x - self.end.x) as f32);
        let y = om * ((self.start.y - self.end.y) as f32);
        (x, y)
    }
}
