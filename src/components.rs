use amethyst::core::ecs::{Component, DenseVecStorage, NullStorage};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TileTransform {
    pub x: i32,
    pub y: i32
}
impl Component for TileTransform {
    type Storage = DenseVecStorage<Self>;
}
impl TileTransform {
    pub fn new (x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }
    pub fn set (&mut self, t: TileTransform) {
        self.x = t.x;
        self.y = t.y;
    }
    pub fn from_ref (t: &TileTransform) -> Self {
        Self {
            x: t.x,
            y: t.y
        }
    }
}



#[derive(Default)]
pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}


pub struct NPC {
    is_enemy: bool
}
impl NPC {
    pub fn new (is_enemy: bool) -> Self {
        Self {
            is_enemy
        }
    }
}
impl Component for NPC {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Collider;
impl Component for Collider {
    type Storage = NullStorage<Self>;
}



pub struct ColliderList {
    colls: Vec<TileTransform>
}
impl ColliderList {
    pub fn new () -> Self {
        Self {
            colls: Vec::new()
        }
    }
    pub fn set (&mut self, c: Vec<TileTransform>) {
        self.colls = c;
    }
    pub fn get (&self) -> Vec<TileTransform> {
        self.colls.clone()
    }
}
impl Component for ColliderList {
    type Storage = DenseVecStorage<Self>;
}