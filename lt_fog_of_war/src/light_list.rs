use lonely_tribes_components::{point_light::PointLight, tile_transform::TileTransform};
use std::ops::{Deref, DerefMut};

pub type LightListVec = (TileTransform, PointLight);

#[derive(Clone, Debug, Default)]
pub struct LightList {
    pub list: Vec<LightListVec>,
}
impl Deref for LightList {
    type Target = Vec<LightListVec>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
impl DerefMut for LightList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}
impl LightList {
    pub fn set(&mut self, list: Vec<LightListVec>) {
        self.list = list;
    }
    pub fn get(&self) -> &[LightListVec] {
        &self.list
    }
}
