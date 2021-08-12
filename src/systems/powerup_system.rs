use crate::{
    components::{
        animator::Animator, colliders::ColliderList, data_holder::EntityHolder, player::Player,
        power_up::PowerUp, tile_transform::TileTransform, win_state::GameWinState,
    },
    systems::move_player::{set_tiletransform_timed, tile_works},
    HEIGHT, WIDTH,
};
use amethyst::core::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use rand::Rng;

///System to handle powerups
pub struct PowerUpSystem;

impl<'s> System<'s> for PowerUpSystem {
    type SystemData = (
        Write<'s, EntityHolder>,
        Write<'s, GameWinState>,
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Animator>,
        Read<'s, ColliderList>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut powers, mut gws, mut tiles, players, mut animators, collider_list, entities): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        for p in powers.clear() {
            match p {
                PowerUp::Portal => {
                    let colliders = collider_list.get();
                    for (tile, _, anim_cmp) in (&mut tiles, &players, &mut animators).join() {
                        let nu_tile = loop {
                            let x = rng.gen_range(0..WIDTH as i32);
                            let y = rng.gen_range(0..HEIGHT as i32);
                            let tile = TileTransform::new(x, y);

                            if tile_works(tile, colliders) {
                                break tile;
                            }
                        };
                        set_tiletransform_timed(tile, nu_tile, anim_cmp, 0.25);
                    }
                }
                PowerUp::Reaper => {
                    let mut new_list = Vec::new();

                    powers.players.iter().for_each(|e| {
                        if rng.gen() {
                            new_list.push(*e);
                        } else {
                            entities.delete(*e).unwrap_or_else(|err| {
                                log::warn!("Error deleting entity with Reaper Powerup: {}", err)
                            });
                        }
                    });
                    powers.players = new_list;
                }
                PowerUp::ScoreChanger => {
                    let factor = rng.gen_range(1.1..=5.0);
                    if rng.gen() {
                        let nu_moves = gws.level_no_of_moves as f32 * factor;
                        gws.level_no_of_moves = nu_moves.round() as i32;
                    } else {
                        let nu_moves = gws.level_no_of_moves as f32 / factor;
                        gws.level_no_of_moves = nu_moves.round() as i32;
                    }
                }
            }
        }
    }
}
