use crate::tag::TriggerType;
use amethyst::core::ecs::{Component, Entity, NullStorage};
use std::ops::Range;
use std::collections::HashMap;
use crate::components::TileTransform;

///Struct to hold a currently being used PowerUp
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PowerUp {
    ///The type of PowerUp
    pub power_up: PowerUpType,

    ///The number of uses that power-up has left
    pub times_left: usize,
}

impl PowerUp {
    pub fn new(power_up: PowerUpType) -> Self {
        Self {
            power_up,
            times_left: power_up.get_uses(),
        }
    }

    pub fn use_it(&mut self) {
        self.times_left -= 1;
    }
    pub fn is_done(&self) -> bool {
        self.times_left <= 0
    }
}

///The type of PowerUp
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerUpType {
    ///Makes it so that players cannot merge or lose
    Shield,
    ///Makes it so that all doors are blocked off
    DoorBlocker,
    ///Makes it so that only one type of player is controlled at once.
    ///(The most recent one to merge)
    Controller, //TODO: allow controller to switch type
    ///Randomises the position of each player (including those already merged)
    Portal,
}
impl PowerUpType {
    ///Get the number of uses that powerup has when it starts
    pub fn get_uses(&self) -> usize {
        match self {
            PowerUpType::Shield => 3,
            PowerUpType::DoorBlocker => 3,
            PowerUpType::Controller => 5,
            PowerUpType::Portal => 1,
        }
    }

    ///Get the trigger id
    pub fn get_trigger_id(&self) -> usize {
        match self {
            Self::Shield => 11,
            Self::DoorBlocker => 12,
            Self::Controller => 13,
            Self::Portal => 14,
        }
    }

    ///Given a trigger id, get the powerup type
    pub fn from_trigger_id(id: &usize) -> Self {
        match id {
            11 => Self::Shield,
            12 => Self::DoorBlocker,
            13 => Self::Controller,
            _ => Self::Portal,
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
        (11..15)
    }
}

///Resource to hold all current powerups
pub struct PowerUpHolder {
    ///Current Powerups
    current: Vec<PowerUp>,

    ///Map of tiletransforms to entities for eventual deletion
    map: HashMap<TileTransform, Entity>
}
impl PowerUpHolder {
    pub fn new() -> Self {
        PowerUpHolder {
            current: Vec::new(),
            map: HashMap::new()
        }
    }

    pub fn prune(&mut self) {
        self.current = self
            .current
            .clone()
            .into_iter()
            .filter(|p| p.is_done())
            .collect();
    }

    pub fn add_pu(&mut self, t: PowerUpType) {
        self.current.push(PowerUp::new(t));
    }

    pub fn get_powerups(&self) -> Vec<PowerUp> {
        self.current.clone()
    }

    pub fn add_entity (&mut self, t: TileTransform, e: Entity) {
        self.map.insert(t, e);
    }
    pub fn remove_entity (&mut self, t: &TileTransform) -> Option<Entity> {
        self.map.remove(t)
    }
}
impl Default for PowerUpHolder {
    fn default() -> Self {
        Self::new()
    }
}