use rand_pcg::Pcg64;
use rand::{SeedableRng, Rng};
use noise::{Fbm, Seedable, NoiseFn};
use crate::level::SpriteRequest;
use crate::{WIDTH, HEIGHT};
use crate::components::tile_transform::TileTransform;
use std::collections::HashMap;

lazy_static! {
    pub static ref BITS_TO_SPRS: HashMap<[bool; 8], SpriteRequest> = {
        // let f = false;
        // let t = true;



        // const BACK: [bool; 8] = [f, f, f, t, t, f, f, f];
        // const BACK_LEFT_CORNER: [bool; u8] = [f, f, f, f, t, f, t, f];

        HashMap::new()
    };
}

pub struct ProceduralGenerator {
    seed: u32,
    perlin: Fbm
}

impl ProceduralGenerator {
    pub fn new (seed: u32) -> Self {
        let p = Fbm::new();

        Self {
            seed,
            perlin: p.set_seed(seed)
        }
    }

    pub fn generate (&self) -> Vec<(usize, usize, SpriteRequest)> {
        let mut rng = Pcg64::seed_from_u64(self.seed as u64);
        let walls: [[bool; HEIGHT as usize]; WIDTH as usize] = Self::generate_walls_bitmap(&mut rng);
        let get_bits = |x: usize, y: usize| {
            let thing_works = |xo: i32, yo: i32| {
                let xtot = x as i32 + xo;
                let ytot = y as i32 + yo;
                if !(0..WIDTH).contains(&xtot) || !(0..HEIGHT).contains(&ytot) {
                    false
                } else {
                    walls[xtot as usize][ytot as usize]
                }
            };

            [thing_works(0, 1), thing_works(1, 1), thing_works(1, 0), thing_works(1, -1), thing_works(0, -1), thing_works(-1, -1), thing_works(-1, 0), thing_works(-1, 1)]
        };

        let mut map = Vec::new();

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                if walls[x][y] {
                    let bits = get_bits(x, y);
                    map.push((x, y, Self::bits_to_sprite_request(bits)));
                }
            }
        }
        map.push((0, 0, SpriteRequest::Player(0)));
        map.push((0, 1, SpriteRequest::Player(0)));

        //TODO: Doors
        //TODO: Greenery/Trees
        //TODO: Players

        map
    }


    fn bits_to_sprite_request (bits: [bool; 8]) -> SpriteRequest {



        SpriteRequest::BackWall
    }

    ///Generates a bitmap for where on the map should be a wall - true is a wall, and false is free
    fn generate_walls_bitmap (rng: &mut Pcg64) -> [[bool; HEIGHT as usize]; WIDTH as usize] {
        let no_rooms = rng.gen_range(3..10);

        //we generate the x and y for no_roo rooms
        let room_max_width = 20;
        let room_max_height = 20;
        let mut gen_room = || -> (TileTransform, TileTransform) {
            let x_pos = rng.gen_range(0..(WIDTH as usize - room_max_width));
            let y_pos = rng.gen_range(0..(HEIGHT as usize - room_max_height));

            let width = rng.gen_range(3..room_max_width);
            let height = rng.gen_range(3..room_max_height);

            let tup: (TileTransform, TileTransform) = ((x_pos, y_pos).into(), (x_pos + width, y_pos + height).into());
            log::info!("Making {:?}", tup);
            tup
        };

        let mut rooms = Vec::new();
        (0..no_rooms).into_iter().for_each(|_| rooms.push(gen_room()));

        let mut map = [[false; HEIGHT as usize]; WIDTH as usize];

        for (top_left, btm_right) in rooms {
            for x in (top_left.x as usize)..=(btm_right.x as usize) {
                map[x][top_left.y as usize] = true;
                map[x][btm_right.y as usize] = true;
            }

            for y in (top_left.y as usize)..=(btm_right.y as usize) {
                map[top_left.x as usize][y] = true;
                map[btm_right.x as usize][y] = true;
            }
        }

        map
    }
}