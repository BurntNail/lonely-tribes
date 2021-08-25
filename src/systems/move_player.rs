use crate::{
    components::{
        animator::{AnimInterpolation, AnimationData, Animator},
        colliders::ColliderList,
        player::Player,
        tile_transform::TileTransform,
        win_state::{GameModeManager, GamePlayingMode, GameState},
    },
    states::paused_state::MovementDisabler,
    Flags, HEIGHT, WIDTH,
};
use amethyst::{
    core::Time,
    ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};
use rand::Rng;
use structopt::StructOpt;
use amethyst::input::VirtualKeyCode;

pub const PLAYER_MOVEMENT_ANIM_LEN: f32 = 0.125;

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
        Read<'s, MovementDisabler>,
        WriteStorage<'s, Animator>,
        Write<'s, MovementType>,
        Write<'s, GameModeManager>,
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
            movement_disabler,
            mut animators,
            mut movement,
            mut gm,
        ): Self::SystemData,
    ) {
        let mut add_to_score = false;
        let mode = gm.current_mode;

        #[allow(unused_variables)]
        let (collision_tiles, trigger_tiles) = {
            let mut collisions = list.get().clone();
            let mut triggers = list.get_triggers().clone();

            if mode == GamePlayingMode::AllTheColliders {
                std::mem::take(&mut triggers)
                    .into_iter()
                    .for_each(|(tt, _)| collisions.push(tt));
            }

            (collisions, triggers)
        };

        let (proposed_tile_addition, actual_movement) = {
            let mut t = TileTransform::default();
            let mut movement = true;

            use VirtualKeyCode::*;
            if input.key_is_down(Up) || input.key_is_down(W) {
                t.y -= 1;
            } else if input.key_is_down(Down) || input.key_is_down(S) {
                t.y += 1;
            } else if input.key_is_down(Left) || input.key_is_down(A) {
                t.x -= 1;
            } else if input.key_is_down(Right) || input.key_is_down(D) {
                t.x += 1;
            } else {
                movement = false;
            }

            (t, movement)
        };

        let proposed_tile_closure = |tile: TileTransform, base_len: f32| {
            let mut rng = rand::thread_rng();
            let mut anim_len = base_len;
            let mut interp = AnimInterpolation::Linear;

            let t = match mode {
                GamePlayingMode::TradeOff => {
                    tile + TileTransform::new(rng.gen_range(-1..=1), rng.gen_range(-1..=1))
                }
                GamePlayingMode::Crazy => {
                    anim_len *= 3.0;
                    interp = AnimInterpolation::ReverseExponential;
                    TileTransform::new(
                        rng.gen_range(0..WIDTH as i32),
                        rng.gen_range(0..HEIGHT as i32),
                    )
                }
                _ => tile + proposed_tile_addition,
            };
            (t, anim_len, interp)
        };

        if let Some((timer, interval)) = &mut movement.movement_timer {
            *timer += time.delta_seconds();

            if timer > interval && !movement_disabler.enabled {
                for (tile, _, anim) in (&mut tiles, &players, &mut animators).join() {
                    let (proposed_tile, anim_len, interp) =
                        proposed_tile_closure(*tile, PLAYER_MOVEMENT_ANIM_LEN);
                    let works = if mode == GamePlayingMode::Nudger {
                        true
                    } else {
                        tile_works(proposed_tile, &collision_tiles) && &proposed_tile != tile
                    };

                    if works && actual_movement {
                        set_tiletransform_with_anim(tile, proposed_tile, anim, anim_len, interp);
                        if mode.adds_to_score() {
                            add_to_score = true;
                        }
                    }
                }

                *timer = 0.0;
            }
        }

        if let Some(can_move) = movement.can_move {
            if !movement_disabler.enabled {
                for (tile, _, anim) in (&mut tiles, &players, &mut animators).join() {
                    let (proposed_tile, anim_len, interp) =
                        proposed_tile_closure(*tile, PLAYER_MOVEMENT_ANIM_LEN);
                    let works = if mode == GamePlayingMode::Nudger {
                        true
                    } else {
                        tile_works(proposed_tile, &collision_tiles) && &proposed_tile != tile
                    };

                    if works && can_move && actual_movement {
                        set_tiletransform_with_anim(tile, proposed_tile, anim, anim_len, interp);
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

///Sets one tiletransform equal to another with the animator, and a given duration
pub fn set_tiletransform_with_anim(
    from: &mut TileTransform,
    to: TileTransform,
    anim: &mut Animator,
    t: f32,
    interp: AnimInterpolation,
) {
    anim.replace_data(AnimationData::new(*from, to, t, interp));
    from.set(to);
}
