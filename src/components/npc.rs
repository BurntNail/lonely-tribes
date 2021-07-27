use amethyst::core::ecs::{Component, DefaultVecStorage};

///Struct for Non-Player Characters.
/// Not yet implemented
#[allow(dead_code)]
pub struct NPC {
    ///Whether or not the NPC is an enemy
    is_enemy: bool,
}
impl NPC {
    ///Constructor for NPC Class
    ///
    ///  - **is_enemy** is a boolean for whether or not the npc is an enemy
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
