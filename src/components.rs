use amethyst::core::ecs::{Component, DefaultVecStorage, DenseVecStorage, NullStorage};
use amethyst::core::math::VecStorage;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TileTransform {
    pub x: i32,
    pub y: i32,
}
impl Component for TileTransform {
    type Storage = DefaultVecStorage<Self>;
}
impl Default for TileTransform {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl TileTransform {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn set(&mut self, t: TileTransform) {
        self.x = t.x;
        self.y = t.y;
    }
    pub fn from_ref(t: &TileTransform) -> Self {
        Self { x: t.x, y: t.y }
    }
}

#[derive(Default)]
pub struct Player {
    pub id: usize,
}
impl Player {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
impl Component for Player {
    type Storage = DefaultVecStorage<Self>;
}

pub struct NPC {
    is_enemy: bool,
}
impl NPC {
    pub fn new(is_enemy: bool) -> Self {
        Self { is_enemy }
    }
}
impl Default for NPC {
    fn default() -> Self {
        Self { is_enemy: false }
    }
}
impl Component for NPC {
    type Storage = DefaultVecStorage<Self>;
}

#[derive(Debug)]
pub struct Collider {
    pub is_trigger: bool,
    pub trigger_id: Option<usize>, //Make it Option<(bool, usize)>
}
impl Default for Collider {
    fn default() -> Self {
        Self {
            is_trigger: false,
            trigger_id: Option::None,
        }
    }
}
impl Collider {
    pub fn new(is_trigger: bool, trigger_id: usize) -> Self {
        Self {
            is_trigger,
            trigger_id: Some(trigger_id),
        }
    }
}
impl Component for Collider {
    type Storage = DefaultVecStorage<Self>;
}

pub struct ColliderList {
    colls: Vec<TileTransform>,
    triggers: Vec<(TileTransform, usize)>,
}
impl ColliderList {
    pub fn new() -> Self {
        Self {
            colls: Vec::new(),
            triggers: Vec::new(),
        }
    }

    pub fn set(&mut self, c: Vec<TileTransform>) {
        self.colls = c;
    }
    pub fn set_triggers(&mut self, t: Vec<(TileTransform, usize)>) {
        self.triggers = t;
    }

    pub fn get(&self) -> Vec<TileTransform> {
        self.colls.clone()
    }
    pub fn get_triggers(&self) -> Vec<(TileTransform, usize)> {
        self.triggers.clone()
    }
}
impl Default for ColliderList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WinStateEnum {
    End { won: bool },
    TBD,
}
impl Default for WinStateEnum {
    fn default() -> Self {
        Self::TBD
    }
}
#[derive(Copy, Clone, Debug)]
pub struct GameWinState {
    pub ws: WinStateEnum,
}
impl Default for GameWinState {
    fn default() -> Self {
        GameWinState {
            ws: WinStateEnum::default(),
        }
    }
}
impl GameWinState {
    pub fn new(won_opt: Option<bool>) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
            },
            Some(won) => Self {
                ws: WinStateEnum::End { won },
            },
        }
    }
    pub fn new_ref(won_opt: Option<&bool>) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
            },
            Some(won_ref) => {
                let mut won = false;
                if won_ref == &true {
                    won = true;
                }
                Self {
                    ws: WinStateEnum::End { won },
                }
            }
        }
    }
}
