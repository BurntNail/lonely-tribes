use amethyst::core::ecs::{Component, DenseVecStorage};

///Component for Moving text up and down.
pub struct TextWobble {
    ///The distance to move around
    pub distance: f32,
    ///The starting y position
    pub old_y: f32,

    ///The duration of one full cycle
    pub duration: f32,
    ///The current time in the cycle
    pub current_time: f32,
}

impl TextWobble {
    ///Constructor for TextWobble
    ///
    ///  - **distance** is the distance
    ///  - **y** is the starting y
    ///  - **duration** is the duration
    pub fn new(distance: f32, y: f32, duration: f32) -> Self {
        Self {
            distance,
            old_y: y,
            duration,
            current_time: 0.0,
        }
    }
}

impl Component for TextWobble {
    type Storage = DenseVecStorage<Self>;
}
