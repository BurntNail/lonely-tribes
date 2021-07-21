use crate::level::SpriteRequest;
use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Copy, Clone, Debug)]
pub enum Tag {
    NPC {is_enemy: bool},
    Player,
    Collision,
    Floor,
    Weapon
}

impl Tag {
    pub fn from_spr (spr: SpriteRequest) -> Self {
        use SpriteRequest::*;
        use Tag::*;
        match spr {
            SpriteRequest::Player => Tag::Player,
            Orc => NPC {is_enemy: true},
            Axe | Bow | Arrow => Weapon,
            Blank | Shrubbery | DarkShrubbery => Floor,
            _ => Collision
        }
    }
}


#[derive(Debug, Clone)]
pub struct TagComponent {
    pub tags: Vec<Tag>
}
impl TagComponent {
    pub fn new (tags: Vec<Tag>) -> Self {
        Self {
            tags
        }
    }
    pub fn add_tag (&mut self, t: Tag) {
        self.tags.push(t);
    }
}
impl Component for TagComponent {
    type Storage = DenseVecStorage<Self>;
}