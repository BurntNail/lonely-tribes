use crate::{
    components::{ColliderList, GameWinState, Player, PowerUp, PowerUpHolder, TileTransform},
    systems::move_player::tile_is_bad,
    HEIGHT, WIDTH,
};
use amethyst::core::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use rand::Rng;

pub struct PowerUpSystem;

impl<'s> System<'s> for PowerUpSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, PowerUpHolder>,
        Write<'s, GameWinState>,
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, ColliderList>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut powers, mut gws, mut tiles, players, collider_list, entities): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        for p in powers.clear() {
            log::info!("DOING {:?}", p);
            match p {
                PowerUp::Portal => {
                    let colliders = collider_list.get();
                    for (tile, _) in (&mut tiles, &players).join() {
                        let nu_tile = loop {
                            let x = rng.gen_range(0..WIDTH as i32);
                            let y = rng.gen_range(0..HEIGHT as i32);
                            let tile = TileTransform::new(x, y);

                            if !tile_is_bad(tile, &colliders) {
                                break tile;
                            }
                        };
                        tile.set(nu_tile);
                    }
                }
                PowerUp::Reaper => {
                    let mut new_list = Vec::new();

                    powers.players.clone().into_iter().for_each(|e| {
                        if rng.gen() {
                            new_list.push(e);
                        } else {
                            entities.delete(e).unwrap_or_else(|err| {
                                log::warn!("Error deleting entity with Reaper Powerup: {}", err)
                            });
                        }
                    });
                    powers.players = new_list;
                }
                PowerUp::ScoreChanger => {
                    let factor = rng.gen_range(2..=5);
                    if rng.gen() {
                        gws.level_no_of_moves *= factor;
                    } else {
                        gws.level_no_of_moves /= factor;
                    }
                }
            }
        }
    }
}
