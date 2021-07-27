use crate::components::GameWinState;
use crate::{
    components::{ColliderList, Player, TileTransform},
    {HEIGHT, WIDTH},
};
use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub const INTERVAL: f32 = 0.05;

///System for capturing player movement, and collision
#[derive(SystemDesc)]
pub struct MovePlayerSystem {
    timer: f32,
}

impl Default for MovePlayerSystem {
    fn default() -> Self {
        Self { timer: 0.0 }
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
        self.timer += time.delta_seconds();
        let mut actual_movement = false;

        if self.timer > INTERVAL {
            let collision_tiles = list.get();

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

            self.timer = 0.0;
        }
    }
}
