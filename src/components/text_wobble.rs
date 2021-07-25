use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct TextWobble {
    pub distance: f32,
    pub old_y: f32,

    pub duration: f32,
    pub current_time: f32,
}

impl TextWobble {
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
