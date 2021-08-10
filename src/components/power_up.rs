use crate::tag::TriggerType;
use serde::{Deserialize, Serialize};
use std::ops::Range;

///The type of PowerUp
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerUp {
    ///Randomises the position of each player (including those already merged)
    Portal,
    ///Kills half of all players randomly
    Reaper,
    ///50/50 chance of lowering or increasing your score
    ScoreChanger,
}
impl PowerUp {
    ///Get the trigger id
    pub fn get_trigger_id(&self) -> usize {
        match self {
            Self::Portal => 12,
            Self::Reaper => 13,
            Self::ScoreChanger => 14,
        }
    }

    ///Given a trigger id, get the powerup type
    pub fn from_trigger_id(id: &usize) -> Self {
        match id {
            12 => Self::Portal,
            13 => Self::Reaper,
            _ => Self::ScoreChanger,
        }
    }
    ///Turns a usize to a TriggerType
    pub fn from_trigger_id_tt(id: &usize) -> TriggerType {
        TriggerType::Powerup(Self::from_trigger_id(id))
    }

    ///Get a range including all of the trigger ids for powerups
    ///
    /// Useful for checking if an trigger ID is a powerup
    pub fn trigger_id_range() -> Range<usize> {
        12..15
    }
}
