///Type of Interpolation for Animation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnimInterpolation {
    ReverseExponential,
    #[allow(dead_code)]
    Linear,
}

///constant for reverse exponential
const A: f32 = 2.0;
///constant for reverse exponential
const B: f32 = -6.5;

impl Default for AnimInterpolation {
    fn default() -> Self {
        Self::ReverseExponential
    }
}
impl AnimInterpolation {
    ///Turns a 0.0-1.0 value into another 0.0-1.0 value, using the interpolation method of self
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

///Gets the current percentage complete of the animation
pub fn get_offset_multiplier(
    time_elapsed: f32,
    total_time: f32,
    interpolation: AnimInterpolation,
) -> f32 {
    let base = time_elapsed / total_time;

    let val: f32 = {
        let str_version = format!("{:03}", interpolation.get_val_from_pctg(base)); //get down to 3dp
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
