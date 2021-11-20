use amethyst::{
    core::ecs::{Component, DefaultVecStorage, DenseVecStorage},
    renderer::resources::Tint,
};

///Component for an entity to be a Point Light
///
/// This works using amethyst Tint objects to 'shade' objects
#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    ///Radius of point light
    pub radius: u32,
}
impl PointLight {
    ///Constructor
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

///Component for a Tint Override
///
///Used when lighting, so that the shadow lighting doesn't mess with the custom tint
#[derive(Copy, Clone, Debug)]
pub struct TintOverride(pub Tint);
impl Component for TintOverride {
    type Storage = DenseVecStorage<Self>;
}
