use amethyst::core::ecs::{Component, DefaultVecStorage};

#[derive(Default)]
pub struct Player {
    pub id: usize,
}
impl Player {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
impl Component for Player {
    type Storage = DefaultVecStorage<Self>;
}
