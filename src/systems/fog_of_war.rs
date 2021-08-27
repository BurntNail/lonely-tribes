use crate::components::{point_light::PointLight, tile_transform::TileTransform};
use amethyst::{
    core::ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::resources::Tint,
};
use std::ops::{Deref, DerefMut};

pub struct FogOfWarSystem;

impl<'s> System<'s> for FogOfWarSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        Read<'s, LightList>,
        WriteStorage<'s, Tint>,
    );

    fn run(&mut self, (tiles, lights, mut tints): Self::SystemData) {
        let list = lights.get();
        let is_lighted = |t: &TileTransform| {
            let mut factor = None;
            list.iter().for_each(|(ti, l)| {
                let distance = t.distance(ti);
                let rad = l.radius as f32;
                if distance < rad {
                    let try_factor = 1.0 - distance / rad;
                    let current = factor.unwrap_or(0.0);
                    if try_factor >= current {
                        factor = Some(try_factor);
                    }
                }
            });
            factor
        };

        for (tile, tint) in (&tiles, &mut tints).join() {
            if let Some(factor) = is_lighted(tile) {
                tint.0.red = factor;
                tint.0.green = factor;
                tint.0.blue = factor;
                tint.0.alpha = factor;
            } else {
                tint.0.red = 0.0;
                tint.0.green = 0.0;
                tint.0.blue = 0.0;
                tint.0.alpha = 0.0;
            }
        }
    }
}

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

pub struct LightListSystem;

impl<'s> System<'s> for LightListSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, PointLight>,
        Write<'s, LightList>,
    );

    fn run(&mut self, (tiles, lights, mut light_list): Self::SystemData) {
        let mut list = Vec::new();
        for (t, p) in (&tiles, &lights).join() {
            list.push((*t, *p));
        }
        light_list.set(list);
    }
}
