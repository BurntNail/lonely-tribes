use crate::level::SpriteRequest;
use crate::components::PowerUpType;

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
    Powerup(PowerUpType)
}
impl TriggerType {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Door => 10,
            Self::Powerup(t) => t.get_trigger_id(),
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
            Axe  => Weapon,
            Blank | Shrubbery | DarkShrubbery => Floor,
            _ => Collision,
        }
    }
}
