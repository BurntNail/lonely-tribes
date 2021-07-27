use amethyst::core::ecs::{Component, NullStorage};

///Marker component to mark UiText objects to set the score
#[derive(Default)]
pub struct Score;

impl Component for Score {
    type Storage = NullStorage<Self>;
}
