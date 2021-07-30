use amethyst::core::ecs::{Component, DefaultVecStorage};

///Struct for Non-Player Characters.
/// Not yet implemented
#[allow(dead_code)]
pub struct NonPlayerCharacter {
    ///Whether or not the NPC is an enemy
    is_enemy: bool,
}
impl NonPlayerCharacter {
    ///Constructor for NPC Class
    ///
    ///  - **is_enemy** is a boolean for whether or not the npc is an enemy
    pub fn new(is_enemy: bool) -> Self {
        Self { is_enemy }
    }
}
impl Default for NonPlayerCharacter {
    fn default() -> Self {
        Self { is_enemy: false }
    }
}
impl Component for NonPlayerCharacter {
    type Storage = DefaultVecStorage<Self>;
}
