use crate::{components::PowerUpType, level::SpriteRequest};

#[derive(Copy, Clone, Debug)]
pub enum Tag {
    NPC { is_enemy: bool },
    Player(usize),
    Collision,
    Floor,
    Weapon,
    Trigger(TriggerType),
}
#[derive(Copy, Clone, Debug)]
pub enum TriggerType {
    Door,
    Powerup(PowerUpType),
    Player(usize),
}
impl TriggerType {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Door => 10,
            Self::Powerup(t) => t.get_trigger_id(),
            Self::Player(u) => *u,
        }
    }
    pub fn from_id(id: &usize) -> Self {
        use TriggerType::*;
        match id {
            10 => Door,
            0 => Player(0),
            1 => Player(1),
            2 => Player(2),
            3 => Player(3),
            _ => PowerUpType::from_trigger_id_tt(id),
        }
    }
}

impl Tag {
    pub fn from_spr(spr: SpriteRequest) -> Self {
        use SpriteRequest::*;
        use Tag::*;
        match spr {
            SpriteRequest::Player(id) => Self::Player(id),
            SpriteRequest::Door => Self::Trigger(TriggerType::Door),
            SpriteRequest::PowerUp(t) => Self::Trigger(TriggerType::Powerup(t)),
            Orc => NPC { is_enemy: true },
            Axe => Weapon,
            Blank | Shrubbery | DarkShrubbery => Floor,
            _ => Collision,
        }
    }
}
