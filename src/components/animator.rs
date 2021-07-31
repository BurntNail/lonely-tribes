use crate::{components::TileTransform, systems::UpdateTileTransforms};
use amethyst::core::{
    ecs::{Component, DenseVecStorage, Entity},
    Transform,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Animator {
    pub start: TileTransform,
    pub end: TileTransform,
    pub difference: TileTransform,

    pub total_time: f32,
    pub time_elapsed: f32,
    pub entity: Option<Entity>,
}

impl Component for Animator {
    type Storage = DenseVecStorage<Self>;
}

impl Animator {
    pub fn new(start: TileTransform, end: TileTransform, total_time: f32) -> Self {
        Self {
            start,
            end,
            difference: start.minus_from_self(end),
            total_time,
            ..Default::default()
        }
    }
}
