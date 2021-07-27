use crate::components::tile_transform::TileTransform;
use amethyst::core::ecs::{Component, DefaultVecStorage};

///Component to mark an entity as a collider
#[derive(Debug)]
pub struct Collider {
    ///A variable of Option type to check if it is a trigger
    /// If it is None, then it is a normal collider,
    /// If it is Some, then the usize is the trigger ID
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
    ///Constructor for a Trigger Collider
    /// For non-trigger Colliders, use the default
    ///
    ///  - **trigger_id** is the id for the trigger
    pub fn new(trigger_id: usize) -> Self {
        Self {
            trigger: Some(trigger_id),
        }
    }
}
impl Component for Collider {
    type Storage = DefaultVecStorage<Self>;
}

///Struct for holding a list of all colliders, and triggers
pub struct ColliderList {
    ///List of all colliders
    colls: Vec<TileTransform>,
    ///List of all Triggers
    triggers: Vec<(TileTransform, usize)>,
}
impl ColliderList {
    ///Constructor for ColliderList
    /// Initialises both lists inside as empty vectors
    pub fn new() -> Self {
        Self {
            colls: Vec::new(),
            triggers: Vec::new(),
        }
    }

    ///Sets the list of colliders
    pub fn set(&mut self, c: Vec<TileTransform>) {
        self.colls = c;
    }
    ///Sets the list of triggers
    pub fn set_triggers(&mut self, t: Vec<(TileTransform, usize)>) {
        self.triggers = t;
    }

    ///Gets a clone of the list of colliders
    pub fn get(&self) -> Vec<TileTransform> {
        self.colls.clone()
    }
    ///Gets a clone of the list of triggers
    pub fn get_triggers(&self) -> Vec<(TileTransform, usize)> {
        self.triggers.clone()
    }
}
impl Default for ColliderList {
    fn default() -> Self {
        Self::new()
    }
}
