use lonely_tribes_components::{point_light::PointLight, tile_transform::TileTransform};
use lonely_tribes_lib::{CONFIG, HEIGHT, WIDTH};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, sync::mpsc::channel};

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

const AREA_THRESHOLD: f32 = 2.0;

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
            .filter(|tile_start| {
                let tile_start: &TileTransform = tile_start;
                let x1 = tile_start.x as f32;
                let y1 = tile_start.y as f32;
                let x3 = light.x as f32;
                let y3 = light.y as f32;

                let mut works = true;

                //https://www.geeksforgeeks.org/program-check-three-points-collinear/
                //first method
                for coll in colls {
                    let x2 = coll.x as f32;
                    let y2 = coll.y as f32;

                    let val = ((y3 - y2) * (x2 - x1) -
                        (y2 - y1) * (x3 - x2));
                    log::info!("v = {}", val);

                    /*let collider_in_right_direction = {
                        let xd_t = light.x - tile_start.x;
                        let xd_c = light.x - coll.x;

                        let yd_t = light.y - tile_start.y;
                        let yd_c = light.y - coll.y;

                        let x_works = {
                            if xd_t < 0 {
                                xd_c > xd_t
                            } else {
                                xd_c < xd_t
                            }
                            // xd_c > xd_t
                        };
                        let y_works = {
                            if yd_t < 0 {
                                yd_c < yd_t
                            } else {
                                yd_c > yd_t
                            }
                            // yd_c < yd_t
                        };
                        x_works && y_works
                    };*/

                    if val.abs() < AREA_THRESHOLD
                        // && collider_in_right_direction
                        && tile_start.distance(coll) > f32::sqrt(2.0)
                    {
                        // log::info!(
                        //     "for {}, with light {}, blocked by coll {}, with val {}",
                        //     tile_start,
                        //     light,
                        //     coll,
                        //     val
                        // );
                        works = false;
                    }
                }

                works
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
        let fow_enabled = CONFIG.flags.fow_enabled();

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

        self.current = Some(LightingData {
            tints: hm.clone(),
            lights: converted_lights,
            colls: Vec::from(colls),
        });

        hm
    }
}
