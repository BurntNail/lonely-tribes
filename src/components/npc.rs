use amethyst::core::ecs::{Component, DefaultVecStorage};

#[allow(dead_code)]
pub struct NPC {
    is_enemy: bool,
}
impl NPC {
    pub fn new(is_enemy: bool) -> Self {
        Self { is_enemy }
    }
}
impl Default for NPC {
    fn default() -> Self {
        Self { is_enemy: false }
    }
}
impl Component for NPC {
    type Storage = DefaultVecStorage<Self>;
}
