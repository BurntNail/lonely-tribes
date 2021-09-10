use rand_pcg::Pcg64;
use rand::{SeedableRng, Rng};
use noise::{Fbm, Seedable, NoiseFn};
use crate::level::SpriteRequest;
use crate::{WIDTH, HEIGHT};
use crate::components::tile_transform::TileTransform;
use std::collections::HashMap;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

pub const PERLIN_SCALE: f64 = 5.0;

///for walls which need more info than 8 bits
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum WallType {
    Back,
    Front,
    Left,
    Right,
}

lazy_static! {
    static ref BITS_TO_SPRS: HashMap<[Option<WallType>; 8], SpriteRequest> = {
        let mut map = HashMap::new();
        let mut a = |list: [Option<WallType>; 8], s: SpriteRequest| {
            map.insert(list, s);
        };


        use SpriteRequest::*;
        let n = None;
        let b = Some(WallType::Back);
        let f = Some(WallType::Front);
        let l = Some(WallType::Left);
        let r = Some(WallType::Right);

        a([n, n, n, b, b, n, n, n], BackWall);
        a([n, n, r, b, r, n, n, n], BackWall);
        a([l, n, n, l, b, n, n, n], BackWall);
        a([n, l, n, n, b, n, n, n], BackWallRightCorner);
        a([n, r, n, b, n, n, n, n], BackWallLeftCorner);

        a([n, n, n, n, f, n, l, n], FrontWallLeftCorner);
        a([n, n, n, f, n, n, r, n], FrontWallRightCorner);
        a([n, n, n, f, f, n, n, n], FrontWall);
        a([n, n, n, l, f, l, n, n], FrontWall);
        a([n, n, n, f, r, n, n, r], FrontWall);

        a([n, l, n, n, n, n, l, n], LeftWall);
        a([n, l, n, n, n, n, l, b], LeftWall);
        a([n, l, n, n, n, n, l, f], LeftWall);
        a([n, l, b, n, n, n, l, n], LeftWall);
        a([n, l, f, n, n, n, l, n], LeftWall);
        a([n, r, n, n, n, n, r, n], RightWall);
        a([n, r, n, n, n, b, r, n], RightWall);
        a([n, r, n, n, n, f, r, n], RightWall);
        a([b, r, n, n, n, n, r, n], RightWall);
        a([f, r, n, n, n, n, r, n], RightWall);




        map
    };
}

pub struct ProceduralGenerator {
    seed: u32,
}

type Map = Vec<(usize, usize, SpriteRequest)>;

pub const TREE_THRESHOLD: f64 = 0.5;
pub const OVERRIDE_WALL_THRESHOLD: f64 = 0.4;
pub const SHRUBBERY_THRESHOLD: f64 = 0.0;
pub const DOOR_REPLACER_MODIFIER: f64 = 5.0;


impl ProceduralGenerator {
    pub fn new (seed: u32) -> Self {
        Self {
            seed,
        }
    }

    pub fn get(&self) -> Map {
        let mut map = Self::generate_walls_sprs(self.seed as u64);
        Self::generate_plants(self.seed, &mut map);
        map.push((0, 0, SpriteRequest::Player(0)));
        map.push((10, 10, SpriteRequest::Player(0)));
        map
    }

    fn generate_plants (seed: u32, map: &mut Map) {
        let blocked_bits: Vec<(usize, usize)> = map.clone().into_par_iter().filter(|(_, _, spr)| spr != &SpriteRequest::Blank && spr != &SpriteRequest::Door).map(|(x, y, _)| (x, y)).collect();
        let plant_places: Vec<(usize, usize)> = map.clone().into_par_iter().filter(|(_, _, spr)| spr == &SpriteRequest::Door).map(|(x, y, _)| (x, y)).collect();

        let p1 = Fbm::new().set_seed(seed);
        let p2 = Fbm::new().set_seed(seed + 100);
        let p3 = Fbm::new().set_seed(seed / 3); //for overrides

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                let p_val = [x as f64 / PERLIN_SCALE, y as f64 / PERLIN_SCALE];

                let no_1 = p1.get(p_val);
                let no_2 = p2.get(p_val);

                let no_3 = if plant_places.contains(&(x, y)) {
                    if no_1 > SHRUBBERY_THRESHOLD {
                        Some((0, no_1))
                    } else {
                        Some((1, no_2))
                    }
                } else {
                    None
                };

                if no_1 > SHRUBBERY_THRESHOLD || no_2 > SHRUBBERY_THRESHOLD || no_3.is_some() {
                    let can_override = p3.get(p_val) > OVERRIDE_WALL_THRESHOLD;

                    let mut changer = |shrubbery: SpriteRequest, tree_spr: SpriteRequest, v: f64| {
                        if blocked_bits.contains(&(x, y)) && can_override {
                            map.push((x, y, tree_spr));
                        } else {
                            if v > TREE_THRESHOLD {
                                map.push((x, y, tree_spr));
                            } else if v > SHRUBBERY_THRESHOLD {
                                map.push((x, y, shrubbery));
                            }
                        }
                    };

                    if let Some((t, val)) = no_3 {
                        if t == 0 {
                            changer(SpriteRequest::Shrubbery, SpriteRequest::Tree, val.abs() * DOOR_REPLACER_MODIFIER);
                        } else {
                            changer(SpriteRequest::DarkShrubbery, SpriteRequest::WarpedTree, val.abs() * DOOR_REPLACER_MODIFIER);
                        }

                    }
                    if no_1 > 0.0 {
                        changer(SpriteRequest::Shrubbery, SpriteRequest::Tree, no_1);
                    }
                    if no_2 > 0.0 {
                        changer(SpriteRequest::DarkShrubbery, SpriteRequest::WarpedTree, no_2);
                    }
                }
            }
        }
    }

    fn generate_walls_sprs (seed: u64) -> Map {
        let mut rng = Pcg64::seed_from_u64(seed as u64);
        let walls: [[Option<WallType>; HEIGHT as usize]; WIDTH as usize] = Self::generate_walls(&mut rng);
        println!("{:?}", walls);

        let get_bits = |x: usize, y: usize| {

            let thing_works = |xo: i32, yo: i32| {
                let xtot = x as i32 + xo;
                let ytot = y as i32 + yo;
                if !(0..WIDTH).contains(&xtot) || !(0..HEIGHT).contains(&ytot) {
                    None
                } else {
                    walls[xtot as usize][ytot as usize]
                }
            };


            [thing_works(-1, 1), thing_works(0, 1), thing_works(1, 1), thing_works(-1, 0), thing_works(1, 0), thing_works(-1, -1), thing_works(0, -1), thing_works(1, -1)]
        };

        let mut map = Vec::new();

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                if walls[x][y].is_some() {
                    let bits = get_bits(x, y);

                    let spr = *BITS_TO_SPRS.get(&bits).unwrap_or(&SpriteRequest::Door);
                    let res = (x, y, spr);

                    map.push(res);
                }
            }
        }

        map
    }

    fn generate_walls(rng: &mut Pcg64) -> [[Option<WallType>; HEIGHT as usize]; WIDTH as usize] {
        let no_rooms = rng.gen_range(3..=8);

        //we generate the x and y for no_roo rooms
        let room_max_width = 20;
        let room_max_height = 20;
        let mut gen_room = || -> (TileTransform, TileTransform) {
            let x_pos = rng.gen_range(0..(WIDTH as usize - room_max_width));
            let y_pos = rng.gen_range(0..(HEIGHT as usize - room_max_height));

            let width = rng.gen_range(4..room_max_width);
            let height = rng.gen_range(4..room_max_height);

            let tup: (TileTransform, TileTransform) = ((x_pos, y_pos).into(), (x_pos + width, y_pos + height).into());
            log::info!("Making {:?}", tup);
            tup
        };

        let mut rooms = Vec::new();
        (0..no_rooms).into_iter().for_each(|_| rooms.push(gen_room()));

        let mut map = [[None; HEIGHT as usize]; WIDTH as usize];

        for (top_left, btm_right) in rooms {
            for x in (top_left.x as usize)..=(btm_right.x as usize) {
                map[x][top_left.y as usize] = Some(WallType::Back);
                map[x][btm_right.y as usize] = Some(WallType::Front);
            }

            for y in (top_left.y as usize)..=(btm_right.y as usize) {
                map[top_left.x as usize][y] = Some(WallType::Left);
                map[btm_right.x as usize][y] = Some(WallType::Right);
            }
        }

        map
    }
}