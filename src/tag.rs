use crate::level::SpriteRequest;

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
}
impl TriggerType {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Door => 10,
        }
    }
}

impl Tag {
    pub fn from_spr(spr: SpriteRequest) -> Self {
        use SpriteRequest::*;
        use Tag::*;
        // use TriggerType::*;
        match spr {
            SpriteRequest::Player(id) => Tag::Player(id),
            SpriteRequest::Door => Tag::Trigger(TriggerType::Door),
            Orc => NPC { is_enemy: true },
            Axe | Bow | Arrow => Weapon,
            Blank | Shrubbery | DarkShrubbery => Floor,
            _ => Collision,
        }
    }
}
