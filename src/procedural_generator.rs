use rand_pcg::Pcg64;
use rand::{SeedableRng, Rng};
use noise::{Fbm, Seedable, NoiseFn};
use crate::level::SpriteRequest;
use crate::{WIDTH, HEIGHT};
use crate::components::tile_transform::TileTransform;

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

    pub fn generate (&self) -> Vec<Vec<SpriteRequest>> {
        let mut rng = Pcg64::seed_from_u64(self.seed as u64);
        let walls: [[bool; WIDTH as usize]; HEIGHT as usize] = Self::generate_walls_bitmap(&mut rng);

        let mut map = vec![vec![SpriteRequest::Blank; WIDTH as usize]; HEIGHT as usize];

        log::info!("Changing");
        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                if walls[y][x] {
                    map[y][x] = SpriteRequest::BackWall;
                }
            }
        }
        log::info!("{:?}", map);

        map
    }

    ///Generates a bitmap for where on the map should be a wall - true is a wall, and false is free
    fn generate_walls_bitmap (rng: &mut Pcg64) -> [[bool; WIDTH as usize]; HEIGHT as usize] {
        //we generate the x and y for 3 rooms
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

        let rooms = vec![gen_room(), gen_room(), gen_room()];

        let mut map = [[false; WIDTH as usize]; HEIGHT as usize];

        for (top_left, btm_right) in rooms {
            for x in (top_left.x as usize)..(btm_right.x as usize) {
                map[top_left.y as usize][x] = true;
                map[btm_right.y as usize][x] = true;
            }
            for y in (top_left.y as usize)..(btm_right.y as usize) {
                map[y][top_left.x as usize] = true;
                map[y][btm_right.x as usize] = true;
            }
        }
        map
    }
}