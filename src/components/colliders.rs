use crate::components::tile_transform::TileTransform;
use amethyst::core::ecs::{Component, DefaultVecStorage};

#[derive(Debug)]
pub struct Collider {
    pub trigger: Option<usize>,
}
impl Default for Collider {
    fn default() -> Self {
        Self {
            trigger: Option::None,
        }
    }
}
impl Collider {
    pub fn new(is_trigger: bool, trigger_id: usize) -> Self {
        Self {
            trigger: if is_trigger { Some(trigger_id) } else { None },
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
