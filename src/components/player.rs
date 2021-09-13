use amethyst::core::ecs::{Component, DefaultVecStorage};
use serde::{Deserialize, Serialize};

///Struct for Player
#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    ///Holds the player ID for win-checking.
    ///Should be identical to the trigger id on that entity.
    pub id: usize,
    pub no_players: u16
}
impl Player {
    ///Constructor for a Player
    ///
    ///  - **id** is the trigger id for the player
    pub fn new(id: usize) -> Self {
        Self { id, no_players: 0 }
    }
}
impl Component for Player {
    type Storage = DefaultVecStorage<Self>;
}
