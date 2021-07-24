use amethyst::core::ecs::{Component, DefaultVecStorage};

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
