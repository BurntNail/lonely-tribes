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

    ///X offset - measured in camera px, rather than grid
    pub x_offset: i32,
    ///Y offset - measured in camera px, rather than grid
    pub y_offset: i32,
}
impl Component for TileTransform {
    type Storage = DefaultVecStorage<Self>;
}
impl Default for TileTransform {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            x_offset: 0,
            y_offset: 0,
        }
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
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    /// Sets the current x and y using coordinates from another TileTransform
    pub fn set(&mut self, t: Self) {
        self.x = t.x;
        self.y = t.y;
    }

    /// Sets the current x and y offsets using another TileTransform
    pub fn set_offsets(&mut self, offsets: (i32, i32)) {
        self.x_offset = offsets.0;
        self.y_offset = offsets.1;
    }

    ///For adding self and another transform, giving the result in a new TileTransform
    pub fn add_into_new(&self, t: Self) -> Self {
        Self {
            x: self.x + t.x,
            y: self.y + t.y,
            x_offset: self.x_offset + t.x_offset,
            y_offset: self.y_offset + t.y_offset,
        }
    }

    ///Minuses self from another tiletransform to make a new tiletransform
    #[allow(dead_code)]
    pub fn minus_from_self(&self, t: Self) -> Self {
        Self {
            x: self.x - t.x,
            y: self.y - t.y,
            x_offset: self.x_offset - t.x_offset,
            y_offset: self.y_offset - t.x_offset,
        }
    }

    ///Turns all values positive
    #[allow(dead_code)]
    pub fn abs(t: Self) -> Self {
        Self {
            x: t.x.abs(),
            y: t.y.abs(),
            x_offset: t.x_offset.abs(),
            y_offset: t.y_offset.abs(),
        }
    }

    ///Gets magnitude of self
    #[allow(dead_code)]
    pub fn get_magnitude(&self) -> f32 {
        let x = (self.x * 8 + self.x_offset) as f32;
        let y = (self.y * 8 + self.y_offset) as f32;

        (x * x + y * y).sqrt()
    }
}
