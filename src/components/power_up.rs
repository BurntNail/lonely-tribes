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
    pub fn get_self_from_trigger_id(id: usize) -> Self {
        match id {
            11 => Self::Shield,
            12 => Self::DoorBlocker,
            13 => Self::Controller,
            _ => Self::Portal,
        }
    }
}
