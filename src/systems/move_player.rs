use crate::{
    components::{ColliderList, GameWinState, Player, TileTransform},
    Flags, {HEIGHT, WIDTH},
};
use amethyst::{
    core::Time,
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
    );

    fn run(&mut self, (mut tiles, players, input, list, time, mut gws): Self::SystemData) {
        //TODO: This works, but it would be nice if it was all in one if statement

        let mut actual_movement = false;
        let collision_tiles = list.get();

        if let Some((tim, int)) = self.movement_timer {
            let timdt = tim + time.delta_seconds();
            self.movement_timer = Some((timdt, int));

            if timdt > int {
                for (tile, _) in (&mut tiles, &players).join() {
                    let mut proposed_tile = *tile;

                    if input.action_is_down("Up").unwrap_or(false) {
                        proposed_tile.y -= 1;
                        actual_movement = true;
                    } else if input.action_is_down("Down").unwrap_or(false) {
                        proposed_tile.y += 1;
                        actual_movement = true;
                    } else if input.action_is_down("Left").unwrap_or(false) {
                        proposed_tile.x -= 1;
                        actual_movement = true;
                    } else if input.action_is_down("Right").unwrap_or(false) {
                        proposed_tile.x += 1;
                        actual_movement = true;
                    }

                    let mut works = true;
                    for possibility in &collision_tiles {
                        if &proposed_tile == possibility {
                            works = false;
                            break;
                        }
                    }
                    if proposed_tile.x < 0
                        || proposed_tile.y < 0
                        || proposed_tile.x > WIDTH as i32 - 1
                        || proposed_tile.y > HEIGHT as i32 - 1
                    {
                        works = false;
                    }

                    if works {
                        tile.set(proposed_tile);
                    }
                }

                if actual_movement {
                    gws.level_no_of_moves += 1;
                }

                self.movement_timer = Some((0.0, int));
            }
        }

        if let Some(can_move) = self.can_move {
            let mut add_to_score = false;

            for (tile, _) in (&mut tiles, &players).join() {
                let mut proposed_tile = *tile;

                if input.action_is_down("Up").unwrap_or(false) {
                    proposed_tile.y -= 1;
                    actual_movement = true;
                } else if input.action_is_down("Down").unwrap_or(false) {
                    proposed_tile.y += 1;
                    actual_movement = true;
                } else if input.action_is_down("Left").unwrap_or(false) {
                    proposed_tile.x -= 1;
                    actual_movement = true;
                } else if input.action_is_down("Right").unwrap_or(false) {
                    proposed_tile.x += 1;
                    actual_movement = true;
                }

                let mut works = true;
                for possibility in &collision_tiles {
                    if &proposed_tile == possibility {
                        works = false;
                        break;
                    }
                }
                if proposed_tile.x < 0
                    || proposed_tile.y < 0
                    || proposed_tile.x > WIDTH as i32 - 1
                    || proposed_tile.y > HEIGHT as i32 - 1
                {
                    works = false;
                }

                if works && can_move && actual_movement {
                    tile.set(proposed_tile);
                    add_to_score = true;
                }
            }

            if add_to_score {
                gws.level_no_of_moves += 1;
            }

            self.can_move = Some(!actual_movement);
        }
    }
}
