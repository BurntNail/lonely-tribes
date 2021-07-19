use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct TileTransform {
    pub x: usize,
    pub y: usize
}
impl Component for TileTransform {
    type Storage = DenseVecStorage<Self>;
}
impl TileTransform {
    pub fn new (x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }
}