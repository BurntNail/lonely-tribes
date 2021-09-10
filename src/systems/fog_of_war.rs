use crate::{
    components::{
        animations::{
            animation::Animator, interpolation::AnimInterpolation, tint::TintAnimatorData,
        },
        colliders::ColliderList,
        point_light::{PointLight, TintOverride},
        tile_transform::TileTransform,
    },
    config::LTConfig,
    systems::move_player::HELD_INTERVAL,
    HEIGHT, WIDTH,
};
use amethyst::{
    core::ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::resources::Tint,
};
use rayon::{iter::ParallelIterator, prelude::*};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::mpsc::channel,
};

#[derive(Default)]
pub struct FogOfWarSystem {
    cacher: LightCacher,
}

pub const TINT_ANIMATION_TIME: f32 = HELD_INTERVAL;

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
        let (sender, receiver) = channel();

        (-rad..rad)
            .into_par_iter()
            .for_each_with(sender, |sender, i| {
                let i = i as i32;
                if !(light.x + i < 0 || light.x + i >= WIDTH as i32) {
                    let (tx, rx) = channel();

                    (-rad..rad).into_par_iter().for_each_with(tx, |tx, j| {
                        let j = j as i32;
                        if !(light.y + j < 0 || light.y + j >= HEIGHT as i32) {
                            let current_delta_pos = TileTransform::new(i, j);

                            if !(colls.contains(&current_delta_pos)
                                || current_delta_pos.get_magnitude() > rad as f32)
                            {
                                tx.send(current_delta_pos + light).unwrap_or_else(|err| {
                                    log::warn!("Couldn't send position to cells to test: {}", err)
                                });
                            }
                        }
                    });

                    for item in rx.iter() {
                        sender.send(item).unwrap_or_else(|err| {
                            log::warn!("Couldn't send position to cells to test: {}", err)
                        });
                    }
                }
            });

        let cells_to_check: Vec<TileTransform> = receiver.iter().collect();

        let mut list: Vec<TileTransform> = cells_to_check
            .into_par_iter()
            .filter(|t| {
                let t = *t;
                let path = t - light;
                let precision = path.get_magnitude() * 10.0; //TODO: do testing to find best val
                let increment = (path.x as f32 / precision, path.y as f32 / precision);
                let mut current_float_repr = (0.0, 0.0);

                loop {
                    //should hopefully never go on forever
                    current_float_repr.0 += increment.0;
                    current_float_repr.1 += increment.1;

                    let current_pos = TileTransform::from(current_float_repr);
                    let plus_delta = light + current_pos;

                    if colls.contains(&plus_delta) || current_pos == path {
                        break true;
                    }

                    if current_pos > t
                        || current_float_repr.0 > WIDTH as f32
                        || current_float_repr.1 > HEIGHT as f32
                    {
                        break false;
                    }
                }
            })
            .collect();
        list.push(light);

        list
    }

    pub fn get_lighted_cells(
        &mut self,
        lights: &[(TileTransform, PointLight)],
        colls: &[TileTransform],
    ) -> HashMap<TileTransform, f32> {
        // let t = SystemTime::now();
        let fow_enabled = LTConfig::new().flags.fow_enabled();

        if !fow_enabled {
            let mut hm = HashMap::new();
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    hm.insert(TileTransform::from((x, y)), 1.0);
                }
            }

            return hm;
        }

        let converted_lights = lights.iter().map(|(t, _)| *t).collect();
        let converted_colls = Vec::from(colls);

        if let Some(data) = &self.current {
            if data.lights == converted_lights && data.colls == converted_colls {
                return data.tints.clone();
            }
        }
        // log::info!("Now done checking for cache: {:?}", t.elapsed());

        let (base_sender, base_receiver) = channel();

        lights
            .par_iter()
            .for_each_with(base_sender, |sender, (l_t_ref, l)| {
                let l_t = *l_t_ref;
                let (tx, rx) = channel();

                Self::get_lighted_cells_no_cache(l_t, l.radius as i32, colls)
                    .into_par_iter()
                    .for_each_with(tx, |tx, t| {
                        let try_fac = if fow_enabled {
                            let dist = t.distance(l_t_ref);
                            let rad = l.radius as f32;
                            (rad - dist) / rad
                        } else {
                            1.0
                        };

                        tx.send((t, try_fac)).unwrap_or_else(|err| {
                            log::warn!(
                                "Error adding lighting factor to list for tile {}: {}",
                                t,
                                err
                            )
                        });
                    });

                for item in rx.iter() {
                    sender.send(item).unwrap_or_else(|err| {
                        log::warn!("Error adding list to lighting data list {}: {}", l_t, err)
                    });
                }
            });
        // log::info!("done checking lights: {:?}", t.elapsed());

        let mut hm = HashMap::new();

        for (tile, try_fac) in base_receiver.iter() {
            let tile: TileTransform = tile;
            let try_fac: f32 = try_fac;

            let current_fac = hm.remove(&tile).unwrap_or(0.0);
            let mut nu_fac = current_fac + try_fac;
            if nu_fac > 1.0 {
                nu_fac = 1.0;
            }
            hm.insert(tile, nu_fac);
        }

        // log::info!("done combining lights: {:?}", t.elapsed());

        self.current = Some(LightingData {
            tints: hm.clone(),
            lights: converted_lights,
            colls: Vec::from(colls),
        });

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
