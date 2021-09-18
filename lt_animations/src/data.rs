///A trait for all structs to hold data for an animation
pub trait AnimationData {
    ///return type for get_current
    type AnimDataType;

    ///Checks whether the animation is done
    fn is_done(&self) -> bool;
    ///Increases the timer
    fn add_time(&mut self, time_since_last: f32);
    ///Gets the current Self::AnimDataType for the animation (eg. a Tint for the Tint animation system)
    fn get_current(&self) -> Self::AnimDataType;
}
