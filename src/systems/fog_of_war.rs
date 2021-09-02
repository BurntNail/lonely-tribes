use crate::{
    components::{
        animations::{
            animation::Animator, interpolation::AnimInterpolation, tint::TintAnimatorData,
        },
        colliders::ColliderList,
        point_light::{PointLight, TintOverride},
        tile_transform::TileTransform,
    },
    systems::move_player::HELD_INTERVAL,
    HEIGHT, WIDTH,
};
use amethyst::{
    core::ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::resources::Tint,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Default)]
pub struct FogOfWarSystem {
    cacher: LightCacher,
}

const TINT_ANIMATION_TIME: f32 = HELD_INTERVAL;

impl<'s> System<'s> for FogOfWarSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, Tint>,
        Read<'s, LightList>,
        Read<'s, ColliderList>,
        ReadStorage<'s, TintOverride>,
        WriteStorage<'s, Animator<TintAnimatorData>>,
    );

    fn run(
        &mut self,
        (tiles, tints, lights, collider_list, overrides, mut animators): Self::SystemData,
    ) {
        let lighted_cells = self
            .cacher
            .get_lighted_cells(lights.get(), collider_list.get());

        for (tile, tint, anim) in (&tiles, &tints, &mut animators).join() {
            let factor = *lighted_cells.get(tile).unwrap_or(&0.0);
            anim.replace_data(TintAnimatorData::new(
                tint.0.alpha,
                factor,
                None,
                TINT_ANIMATION_TIME,
                AnimInterpolation::Linear,
            ));
        }
        for (tile, tint, t_override, anim) in (&tiles, &tints, &overrides, &mut animators).join() {
            let factor = *lighted_cells.get(tile).unwrap_or(&0.0);
            anim.replace_data(TintAnimatorData::new(
                tint.0.alpha,
                factor,
                Some(t_override.0),
                TINT_ANIMATION_TIME,
                AnimInterpolation::Linear,
            ));
        }
    }
}

#[derive(Default, Clone)]
pub struct LightCacher {
    pub current: Option<LightingData>,
}
#[derive(Clone, Default)]
pub struct LightingData {
    pub tints: HashMap<TileTransform, f32>,
    pub lights: Vec<TileTransform>,
    pub colls: Vec<TileTransform>,
}
impl PartialEq for LightingData {
    fn eq(&self, other: &Self) -> bool {
        self.lights == other.lights && self.colls == other.colls
    }
}
impl Eq for LightingData {}

impl LightCacher {
    fn get_lighted_cells_no_cache(
        light: TileTransform,
        rad: i32,
        colls: &[TileTransform],
    ) -> Vec<TileTransform> {
        let mut list = vec![light];

        let mut current_delta_pos = TileTransform::default();
        let mut cells_to_test = Vec::new();
        for i in -rad..rad as i32 {
            if light.x + i < 0 || light.x + i >= WIDTH as i32 {
                continue;
            }
            for j in -rad..rad as i32 {
                if light.y + j < 0 || light.y + j >= HEIGHT as i32 {
                    continue;
                }

                current_delta_pos.x = i;
                current_delta_pos.y = j;

                if colls.contains(&current_delta_pos)
                    || current_delta_pos.get_magnitude() > rad as f32
                {
                    continue;
                }

                cells_to_test.push(current_delta_pos + light);
            }
        }

        let mut current_float_repr = (0.0, 0.0);

        cells_to_test.into_iter().for_each(|t| {
            let path = t - light;
            let precision = path.get_magnitude() * 10.0;
            let increment = (path.x as f32 / precision, path.y as f32 / precision);
            current_float_repr = (0.0, 0.0);

            let worked = loop {
                current_float_repr.0 += increment.0;
                current_float_repr.1 += increment.1;

                let current_pos = TileTransform::from(current_float_repr);
                let plus_delta = light + current_pos;

                if colls.contains(&plus_delta) || current_pos == path {
                    break Some(plus_delta);
                }

                if current_pos > t {
                    break None;
                }
            };

            if let Some(w) = worked {
                list.push(w);
            }
        });

        list
    }

    pub fn get_lighted_cells(
        &mut self,
        lights: &[(TileTransform, PointLight)],
        colls: &[TileTransform],
    ) -> HashMap<TileTransform, f32> {
        let converted_lights = {
            let mut v = Vec::new();
            lights.iter().for_each(|(t, _)| v.push(*t));
            v
        };
        let converted_colls = Vec::from(colls);
        if let Some(data) = &self.current {
            if data.lights == converted_lights && data.colls == converted_colls {
                return data.tints.clone();
            }
        }

        let mut hm = HashMap::new();

        lights.iter().for_each(|(l_t_ref, l)| {
            let l_t = *l_t_ref;
            Self::get_lighted_cells_no_cache(l_t, l.radius as i32, colls)
                .into_iter()
                .for_each(|t| {
                    let dist = t.distance(l_t_ref);
                    let rad = l.radius as f32;
                    let try_fac = (rad - dist) / rad;

                    let current_fac = hm.remove(&t).unwrap_or(0.0);
                    let mut nu_fac = current_fac + try_fac;
                    if nu_fac > 1.0 {
                        nu_fac = 1.0;
                    }

                    hm.insert(t, nu_fac);
                });
        });

        let new_data = LightingData {
            tints: hm.clone(),
            lights: converted_lights,
            colls: Vec::from(colls),
        };
        self.current = Some(new_data);

        hm
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
