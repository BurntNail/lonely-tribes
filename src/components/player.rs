use amethyst::core::ecs::{Component, DefaultVecStorage};
use serde::{Serialize, Deserialize};

///Struct for Player
#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    ///Holds the player ID for win-checking.
    ///Should be identical to the trigger id on that entity.
    pub id: usize,
}
impl Player {
    ///Constructor for a Player
    ///
    ///  - **id** is the trigger id for the player
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
impl Component for Player {
    type Storage = DefaultVecStorage<Self>;
}
