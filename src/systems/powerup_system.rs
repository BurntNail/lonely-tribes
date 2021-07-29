// match trigger_type {
// TriggerType::Door => {}
// TriggerType::Powerup(powerup_type) => {
// if powerup_type == PowerUpType::Portal {
// works = false;
//
// let rand = || {
// loop {
// let x = rand::thread_rng().gen_range(0..WIDTH as i32);
// let y = rand::thread_rng().gen_range(0..HEIGHT as i32);
// let tt = TileTransform::new(x, y);
//
// if !tile_is_bad(tt, &collision_tiles) {
// break tt;
// }
// }
// };
//
// for (player_pos) in
// }
// }
// }

use crate::components::{Player, PowerUpHolder, TileTransform, PowerUp};
use amethyst::core::ecs::{Entities, Read, ReadStorage, System, Write, WriteStorage};

pub struct PowerUpSystem;

impl<'s> System<'s> for PowerUpSystem {
    type SystemData = (
        Write<'s, PowerUpHolder>,
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (mut powers, mut tiles, players): Self::SystemData) {
        // for p in &mut powers.get_powerups() {
        //     match p {
        //         &_ => {}
        //     }
        // }
    }
}
