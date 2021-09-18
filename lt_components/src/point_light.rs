use amethyst::{
    core::ecs::{Component, DefaultVecStorage, DenseVecStorage},
    renderer::resources::Tint,
};

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub radius: u32,
}
impl PointLight {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}
impl Default for PointLight {
    fn default() -> Self {
        Self { radius: 1 }
    }
}
impl Component for PointLight {
    type Storage = DefaultVecStorage<Self>;
}

#[derive(Copy, Clone, Debug)]
pub struct TintOverride(pub Tint);
impl Component for TintOverride {
    type Storage = DenseVecStorage<Self>;
}
