///Type of Interpolation for Animation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnimInterpolation {
    ///A reverse exponential curve
    ///
    /// https://www.desmos.com/calculator/sbq8tbhr9d
    ReverseExponential,
    #[allow(dead_code)]
    ///A linear curve - f(x) = x
    Linear,
}

///constant for reverse exponential
const A: f32 = 4.0;

impl Default for AnimInterpolation {
    fn default() -> Self {
        Self::ReverseExponential
    }
}
impl AnimInterpolation {
    ///Turns a 0.0-1.0 value into another 0.0-1.0 value, using the interpolation method of self
    pub fn get_val_from_pctg(&self, pctg: f32) -> f32 {
        match self {
            AnimInterpolation::ReverseExponential => 1.0 - (1.0 - pctg).powf(A),
            AnimInterpolation::Linear => pctg,
        }
        .abs()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn pctg_tests() {
        let l = |v: f32| AnimInterpolation::Linear.get_val_from_pctg(v);

        assert_eq!(l(100.0), 100.0);
        assert_eq!(l(0.5), 0.5);
        assert_eq!(l(-1.0), 1.0);
    }

    #[test]
    pub fn offset_tests() {
        let l = |v: f32| get_offset_multiplier(v, 1.0, AnimInterpolation::Linear);

        assert_eq!(l(100.0), 100.0);
        assert_eq!(l(0.5), 0.5);
        assert_eq!(l(-1.0), 1.0);
    }
}
