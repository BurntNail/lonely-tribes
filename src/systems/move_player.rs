use crate::{
    components::{
        animator::{AnimationData, Animator},
        colliders::ColliderList,
        data_holder::EntityHolder,
        player::Player,
        power_up::PowerUp,
        tile_transform::TileTransform,
        win_state::{GameModeManager, GamePlayingMode, GameState},
    },
    states::paused_state::MovementDisabler,
    Flags, HEIGHT, WIDTH,
};
use amethyst::{
    core::{ecs::Entities, Time},
    ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};
use structopt::StructOpt;

pub const PLAYER_MOVEMENT_ANIM_LEN: f32 = 0.25;

///System for capturing player movement, and collision
#[derive(Default)]
pub struct MovePlayerSystem;

///Struct for the current movement type
pub struct MovementType {
    ///For new movement system
    ///
    /// If none, we assume to use the legacy.
    /// If the legacy is also none, then we don't move
    pub can_move: Option<bool>,

    ///For legacy system
    ///
    ///Tuple with current time, timer length
    ///
    /// If none, we assume the new system.
    pub movement_timer: Option<(f32, f32)>,
}

impl Default for MovementType {
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
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ColliderList>,
        Read<'s, Time>,
        Write<'s, GameState>,
        Write<'s, EntityHolder>,
        Read<'s, MovementDisabler>,
        WriteStorage<'s, Animator>,
        Write<'s, MovementType>,
        Write<'s, GameModeManager>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut tiles,
            players,
            input,
            list,
            time,
            mut gws,
            mut powers,
            movement_disabler,
            mut animators,
            mut movement,
            mut gm,
            entities,
        ): Self::SystemData,
    ) {
        let mut add_to_score = false;
        let mode = gm.current_mode;

        let collision_tiles = list.get();
        let trigger_tiles = list.get_triggers();

        let (proposed_tile_addition, actual_movement) = {
            let mut t = TileTransform::default();
            let mut movement = true;

            if input.action_is_down("Up").unwrap_or(false) {
                t.y -= 1;
            } else if input.action_is_down("Down").unwrap_or(false) {
                t.y += 1;
            } else if input.action_is_down("Left").unwrap_or(false) {
                t.x -= 1;
            } else if input.action_is_down("Right").unwrap_or(false) {
                t.x += 1;
            } else {
                movement = false;
            }

            (t, movement)
        };

        let mut check_powerups = |proposed_tile: &TileTransform| {
            for (trigger, tt) in trigger_tiles {
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

        if let Some((tim, int)) = movement.movement_timer {
            let timdt = tim + time.delta_seconds();
            movement.movement_timer = Some((timdt, int));

            if timdt > int && !movement_disabler.enabled {
                for (tile, _, anim) in (&mut tiles, &players, &mut animators).join() {
                    let proposed_tile = *tile + proposed_tile_addition;

                    let works = if mode == GamePlayingMode::Nudger {
                        true
                    } else {
                        tile_works(proposed_tile, collision_tiles) && &proposed_tile != tile
                    };

                    if works && actual_movement {
                        set_tiletransform(tile, proposed_tile, anim);
                        check_powerups(&proposed_tile);
                        add_to_score = true;
                    }
                }

                movement.movement_timer = Some((0.0, int));
            }
        }

        if let Some(can_move) = movement.can_move {
            if !movement_disabler.enabled {
                for (tile, _, anim) in (&mut tiles, &players, &mut animators).join() {
                    let proposed_tile = *tile + proposed_tile_addition;

                    let works = if mode == GamePlayingMode::Nudger {
                        true
                    } else {
                        tile_works(proposed_tile, collision_tiles) && &proposed_tile != tile
                    };

                    if works && can_move && actual_movement {
                        set_tiletransform(tile, proposed_tile, anim);
                        check_powerups(&proposed_tile);
                        add_to_score = true;
                    }
                }

                movement.can_move = Some(!actual_movement);
            }
        }

        if add_to_score {
            gws.level_no_of_moves += 1;
            gm.do_move();
        }
    }
}

///Checks whether a proposed TileTransform is in a valid position, given tiles it needs to avoid, using the consts in main.rs for OOB detection.
pub fn tile_works(proposed_tile: TileTransform, collision_tiles: &[TileTransform]) -> bool {
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

///Uses set_tiletransform_timed with a specific delay of 0.05
pub fn set_tiletransform(from: &mut TileTransform, to: TileTransform, anim: &mut Animator) {
    set_tiletransform_timed(from, to, anim, PLAYER_MOVEMENT_ANIM_LEN);
}

///Sets one tiletransform equal to another with the animator, and a given duration
pub fn set_tiletransform_timed(
    from: &mut TileTransform,
    to: TileTransform,
    anim: &mut Animator,
    t: f32,
) {
    let nu_data = AnimationData::new(*from, to, t);
    anim.replace_data(nu_data);

    from.set(to);
}
