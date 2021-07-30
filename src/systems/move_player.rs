use crate::{
    components::{ColliderList, GameWinState, Player, PowerUp, PowerUpHolder, TileTransform},
    Flags, HEIGHT, WIDTH,
};
use amethyst::{
    core::{ecs::Entities, Time},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};
use structopt::StructOpt;

///System for capturing player movement, and collision
#[derive(SystemDesc)]
pub struct MovePlayerSystem {
    ///For new movement system
    ///
    /// If none, we assume to use the legacy.
    /// If the legacy is also none, then we don't move, cos I messed up
    can_move: Option<bool>,
    ///For legacy system
    ///
    ///Tuple with current time, timer length
    ///
    /// If none, we assume the new system.
    movement_timer: Option<(f32, f32)>,
}

impl Default for MovePlayerSystem {
    fn default() -> Self {
        let opts: Flags = Flags::from_args();

        if let Some(interval) = opts.timed_movement {
            Self {
                can_move: None,
                movement_timer: Some((0.0, interval)),
            }
        } else {
            Self {
                can_move: Some(false),
                movement_timer: None,
            }
        }
    }
}

impl<'s> System<'s> for MovePlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ColliderList>,
        Read<'s, Time>,
        Write<'s, GameWinState>,
        Write<'s, PowerUpHolder>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut tiles, players, input, list, time, mut gws, mut powers, entities): Self::SystemData,
    ) {
        let mut actual_movement = false;
        let mut add_to_score = false;
        let collision_tiles = list.get();
        let trigger_tiles = list.get_triggers();

        let proposed_tile_addition = {
            let mut t = TileTransform::default();
            if input.action_is_down("Up").unwrap_or(false) {
                t.y -= 1;
                actual_movement = true;
            } else if input.action_is_down("Down").unwrap_or(false) {
                t.y += 1;
                actual_movement = true;
            } else if input.action_is_down("Left").unwrap_or(false) {
                t.x -= 1;
                actual_movement = true;
            } else if input.action_is_down("Right").unwrap_or(false) {
                t.x += 1;
                actual_movement = true;
            }

            t
        };
        let mut check_powerups = |proposed_tile: &TileTransform| {
            for (trigger, tt) in &trigger_tiles {
                if proposed_tile == trigger {
                    let id = &tt.get_id();
                    if PowerUp::trigger_id_range().contains(id) {
                        let ent = powers.remove_pu_entity(trigger);
                        if let Some(ent) = ent {
                            entities.delete(ent).unwrap_or_else(|err| {
                                log::warn!("Error deleting powerup entity after collision: {}", err)
                            })
                        }

                        powers.add_powerup(PowerUp::from_trigger_id(id));
                    }
                }
            }
        };

        if let Some((tim, int)) = self.movement_timer {
            let timdt = tim + time.delta_seconds();
            self.movement_timer = Some((timdt, int));

            if timdt > int {
                for (tile, _) in (&mut tiles, &players).join() {
                    let proposed_tile = tile.add_into_new(proposed_tile_addition);

                    let works = tile_is_bad(proposed_tile, &collision_tiles);
                    check_powerups(&proposed_tile);

                    if works {
                        tile.set(proposed_tile);
                    }
                }

                if actual_movement {
                    add_to_score = true;
                }

                self.movement_timer = Some((0.0, int));
            }
        }

        if let Some(can_move) = self.can_move {
            for (tile, _) in (&mut tiles, &players).join() {
                let proposed_tile = tile.add_into_new(proposed_tile_addition);

                let works = tile_is_bad(proposed_tile, &collision_tiles);
                check_powerups(&proposed_tile);

                if works && can_move && actual_movement {
                    tile.set(proposed_tile);
                    add_to_score = true;
                }
            }

            self.can_move = Some(!actual_movement);
        }

        if add_to_score {
            gws.level_no_of_moves += 1;
        }
    }
}

pub fn tile_is_bad(proposed_tile: TileTransform, collision_tiles: &[TileTransform]) -> bool {
    let mut res = true;

    if proposed_tile.x < 0
        || proposed_tile.y < 0
        || proposed_tile.x > WIDTH as i32 - 1
        || proposed_tile.y > HEIGHT as i32 - 1
    {
        res = false;
    }

    for possibility in collision_tiles {
        if &proposed_tile == possibility {
            res = false;
        }
    }

    res
}
