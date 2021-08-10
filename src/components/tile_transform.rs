use amethyst::core::ecs::{Component, DefaultVecStorage};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

///Component for transforms which align to the tile grid
///Much easier to manipulate than amethyst Transforms
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
impl Display for TileTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl TileTransform {
    ///Constructor for TileTransform
    /// Takes in an x and a y
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Sets the current x and y using coordinates from another TileTransform
    pub fn set(&mut self, t: Self) {
        self.x = t.x;
        self.y = t.y;
    }
    ///The same as **tile.set()**, but it takes in a reference
    pub fn from_ref(t: &Self) -> Self {
        Self { x: t.x, y: t.y }
    }

    ///For adding self and another transform, giving the result in a new TileTransform
    pub fn add_into_new(&self, t: Self) -> Self {
        Self {
            x: self.x + t.x,
            y: self.y + t.y,
        }
    }

    ///Minuses self from another tiletransform to make a new tiletransform
    #[allow(dead_code)]
    pub fn minus_from_self(&self, t: Self) -> Self {
        Self {
            x: self.x - t.x,
            y: self.y - t.y,
        }
    }

    ///Turns all values positive
    #[allow(dead_code)]
    pub fn abs(t: Self) -> Self {
        Self {
            x: t.x.abs(),
            y: t.y.abs(),
        }
    }

    ///Gets magnitude of self
    #[allow(dead_code)]
    pub fn get_magnitude(&self) -> f32 {
        let x = (self.x * self.x) as f32;
        let y = (self.y * self.y) as f32;
        (x + y).sqrt()
    }
}
