#[cfg(test)]
mod anim_tests {
    use super::super::{
        animation::Animator,
        interpolation::{get_offset_multiplier, AnimInterpolation},
        movement::MovementAnimationData,
        data::AnimationData
    };

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

                assert!(
                    (get_offset_multiplier(a.time_elapsed, a.total_time, a.interpolation) - t)
                        .abs()
                        <= f32::EPSILON
                );
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
