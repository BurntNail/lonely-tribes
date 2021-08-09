use crate::components::{power_up::PowerUp, tile_transform::TileTransform};
use amethyst::core::ecs::Entity;
use std::collections::HashMap;

///Resource to hold all entity related data
pub struct EntityHolder {
    ///Map of tiletransforms to entities for eventual deletion
    pub powerup_entities: HashMap<TileTransform, Entity>,

    ///Vector of players
    pub players: Vec<Entity>,

    ///Vector of Powerups to be Done
    pub powerups: Vec<PowerUp>,
}
impl EntityHolder {
    ///Constructor to initialise all lists with empty hashmaps/vectors
    pub fn new() -> Self {
        EntityHolder {
            powerup_entities: HashMap::new(),
            players: Vec::new(),
            powerups: Vec::new(),
        }
    }

    ///Add a powerup entity (one with a powerup sprite)
    pub fn add_pu_entity(&mut self, t: TileTransform, e: Entity) {
        self.powerup_entities.insert(t, e);
    }
    ///Remove a powerup entity, and return it (likely for deletion after the player uses it)
    pub fn remove_pu_entity(&mut self, t: &TileTransform) -> Option<Entity> {
        self.powerup_entities.remove(t)
    }

    ///Add a player to the entity list
    pub fn add_entity(&mut self, player: Entity) {
        self.players.push(player);
    }
    ///Add a powerup to the list
    ///
    /// If the powerup is already in the list, it does nothing
    pub fn add_powerup(&mut self, p: PowerUp) {
        if !self.powerups.contains(&p) {
            self.powerups.push(p);
        }
    }

    ///Clears out the powerups list, and returns it.
    pub fn clear(&mut self) -> Vec<PowerUp> {
        let mut vec = Vec::new();
        while let Some(p) = self.powerups.pop() {
            vec.push(p);
        }
        vec
    }
}
impl Default for EntityHolder {
    fn default() -> Self {
        Self::new()
    }
}
