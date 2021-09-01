use crate::TILE_WIDTH_HEIGHT;
use amethyst::core::ecs::{Component, DefaultVecStorage};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul, Sub},
};

///Component for transforms which align to the tile grid
///Much easier to manipulate than amethyst Transforms
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TileTransform {
    ///X position - Horizontal
    pub x: i32,
    ///Y Position - Vertical
    pub y: i32,

    ///X offset - measured in camera px, rather than grid. SHOULD ONLY BE USED FOR COSMETICS
    pub x_offset: i32,
    ///Y offset - measured in camera px, rather than grid. SHOULD ONLY BE USED FOR COSMETICS
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

    ///Gets magnitude of self using pythagoras
    pub fn get_magnitude(&self) -> f32 {
        let x = (self.x + (self.x_offset / TILE_WIDTH_HEIGHT)) as f32;
        let y = (self.y + (self.y_offset / TILE_WIDTH_HEIGHT)) as f32;

        (x * x + y * y).sqrt()
    }

    pub fn distance(&self, other: &TileTransform) -> f32 {
        (*self - *other).get_magnitude()
    }
    #[allow(dead_code)]
    pub fn normalised(&self) -> (f32, f32) {
        let mag = self.get_magnitude();
        ((self.x as f32) / mag, (self.y as f32) / mag)
    }
}

//region implementations
impl Add for TileTransform {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            x_offset: self.x_offset + rhs.x_offset,
            y_offset: self.y_offset + rhs.y_offset,
        }
    }
}
impl Sub for TileTransform {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            x_offset: self.x_offset - rhs.x_offset,
            y_offset: self.y_offset - rhs.y_offset,
        }
    }
}
impl AddAssign for TileTransform {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.x_offset += rhs.x_offset;
        self.y_offset += rhs.y_offset
    }
}

impl From<(i32, i32)> for TileTransform {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}
impl From<(f32, f32)> for TileTransform {
    fn from((x, y): (f32, f32)) -> Self {
        (x as i32, y as i32).into()
    }
}
impl Mul<i32> for TileTransform {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut new = Self::new(self.x * rhs, self.y * rhs);
        new.set_offsets((self.x_offset * rhs, self.y_offset * rhs));
        new
    }
}
impl PartialEq for TileTransform {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for TileTransform {}
impl Hash for TileTransform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.x, state);
        Hash::hash(&self.y, state);
    }
}
impl PartialOrd for TileTransform {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_magnitude()
            .abs()
            .partial_cmp(&other.get_magnitude().abs())
    }
}
//endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn add_and_subtract_normal_test() {
        let t1: TileTransform = (5, 1).into();
        let t2: TileTransform = (2, 5).into();

        assert_eq!(t1 + t2, (7, 6).into());
        assert_eq!(t1 - t2, (3, -4).into());
    }
    #[test]
    pub fn add_and_subtract_offset_test() {
        let mut t1: TileTransform = (0, 0).into();
        t1.set_offsets((5, 2));
        let mut t2: TileTransform = (0, 0).into();
        t2.set_offsets((1, 5));

        let mut add = TileTransform::default();
        add.set_offsets((6, 7));
        let mut sub = TileTransform::default();
        sub.set_offsets((4, -3));

        assert_eq!(t1 + t2, add);
        assert_eq!(t1 - t2, sub);
    }

    #[test]
    pub fn magnitude_normal_test() {
        let t: TileTransform = (5, 2).into();
        assert_eq!(t.get_magnitude(), 29.0_f32.sqrt());
    }
    #[test]
    pub fn magnitude_offset_test() {
        let mut t: TileTransform = (5, 2).into();
        t.set_offsets((3, 4));
        assert_eq!(t.get_magnitude(), 35.140625_f32.sqrt());
    }

    #[test]
    pub fn into_test() {
        let base = TileTransform::new(0, 1);
        let t1: TileTransform = (0, 1).into();
        let t2: TileTransform = (0.9, 1.1).into();
        assert_eq!(t1, base);
        assert_eq!(t2, base);
    }
}
