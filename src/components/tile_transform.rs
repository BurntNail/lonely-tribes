use amethyst::core::ecs::{Component, DefaultVecStorage};
use serde::{Deserialize, Serialize};

///Component for transforms which align to the tile grid
/// Much easier to manipulate than amethyst Transforms
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TileTransform {
    ///X position - Horizontal
    pub x: i32,
    ///Y Position - Vertical
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
    ///Constructor for TileTransform
    /// Takes in an x and a y
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Sets the current x and y using coordinates from another TileTransform (arg **t**)
    pub fn set(&mut self, t: TileTransform) {
        self.x = t.x;
        self.y = t.y;
    }
    ///The same as **tile.set()**, but it takes in a reference
    pub fn from_ref(t: &TileTransform) -> Self {
        Self { x: t.x, y: t.y }
    }

    ///For adding self and another transform, giving the result in a new TileTransform
    pub fn add_into_new(&self, t: TileTransform) -> Self {
        Self {
            x: self.x + t.x,
            y: self.y + t.y,
        }
    }
}
