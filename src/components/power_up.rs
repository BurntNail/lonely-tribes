use crate::tag::TriggerType;
use amethyst::core::ecs::{Component, Entity, NullStorage, Entities, Write, WriteStorage};
use std::ops::Range;
use std::collections::HashMap;
use crate::components::{TileTransform, GameWinState};
use rand::Rng;

///The type of PowerUp
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PowerUp {
    ///Randomises the position of each player (including those already merged)
    Portal,
    ///Kills half of all players randomly
    Reaper,
    ///50/50 chance of lowering or increasing your score
    ScoreChanger
}
impl PowerUp {

    ///Get the trigger id
    pub fn get_trigger_id(&self) -> usize {
        match self {
            Self::Portal => 12,
            Self::Reaper => 13,
            Self::ScoreChanger => 14
        }
    }

    ///Given a trigger id, get the powerup type
    pub fn from_trigger_id(id: &usize) -> Self {
        match id {
            12 => Self::Portal,
            13 => Self::Reaper,
            _ => Self::ScoreChanger
        }
    }
    ///Turns a usize to a TriggerType
    pub fn from_trigger_id_tt(id: &usize) -> TriggerType {
        TriggerType::Powerup(Self::from_trigger_id(id))
    }

    ///Get a range including all of the trigger ids for powerups
    ///
    /// Useful for checking if an trigger ID is a powerup
    pub fn trigger_id_range () -> Range<usize> {
        (12..15)
    }
}

///Resource to hold all current powerups
pub struct PowerUpHolder {
    ///Map of tiletransforms to entities for eventual deletion
    pub powerup_entities: HashMap<TileTransform, Entity>,

    ///Vector of players
    pub players: Vec<Entity>,

    ///Vector of Powerups to be Done
    pub powerups: Vec<PowerUp>
}
impl PowerUpHolder { //TODO: Docucomments
    pub fn new() -> Self {
        PowerUpHolder {
            powerup_entities: HashMap::new(),
            players: Vec::new(),
            powerups: Vec::new()
        }
    }

    pub fn add_pu_entity(&mut self, t: TileTransform, e: Entity) {
        self.powerup_entities.insert(t, e);
    }
    pub fn remove_pu_entity(&mut self, t: &TileTransform) -> Option<Entity> {
        self.powerup_entities.remove(t)
    }

    pub fn add_entity (&mut self, player: Entity) {
        self.players.push(player);
    }
    pub fn add_powerup(&mut self, p: PowerUp) {
        if !self.powerups.contains(&p) {
            self.powerups.push(p);
        }
    }

    pub fn clear (&mut self) -> Vec<PowerUp> {
        let mut vec = Vec::new();
        while let Some(p) = self.powerups.pop() {
            vec.push(p);
        }
        vec
    }
}
impl Default for PowerUpHolder {
    fn default() -> Self {
        Self::new()
    }
}