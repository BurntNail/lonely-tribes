use crate::components::GameWinState;
use crate::{
    components::{ColliderList, Player, TileTransform},
    {HEIGHT, WIDTH},
};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

///System for capturing player movement, and collision
#[derive(SystemDesc)]
pub struct MovePlayerSystem {
    can_move: bool,
}

impl Default for MovePlayerSystem {
    fn default() -> Self {
        Self { can_move: false }
    }
}

impl<'s> System<'s> for MovePlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ColliderList>,
        Write<'s, GameWinState>,
    );

    fn run(&mut self, (mut tiles, players, input, list, mut gws): Self::SystemData) {
        let mut actual_movement = false;
        let mut add_to_score = false;
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

            if works && self.can_move && actual_movement {
                tile.set(proposed_tile);
                add_to_score = true;
            }
        }

        if add_to_score {
            gws.level_no_of_moves += 1;
        }

        self.can_move = !actual_movement;
    }
}
