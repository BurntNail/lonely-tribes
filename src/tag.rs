use crate::level::SpriteRequest;
use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Copy, Clone, Debug)]
pub enum Tag {
    NPC { is_enemy: bool },
    Player(usize),
    Collision,
    Floor,
    Weapon,
}

impl Tag {
    pub fn from_spr(spr: SpriteRequest) -> Self {
        use SpriteRequest::*;
        use Tag::*;
        match spr {
            SpriteRequest::Player(id) => Tag::Player(id),
            Orc => NPC { is_enemy: true },
            Axe | Bow | Arrow => Weapon,
            Blank | Shrubbery | DarkShrubbery => Floor,
            _ => Collision,
        }
    }
}
