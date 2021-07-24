use crate::components::{Collider, ColliderList, Player, TileTransform, GameWinState, WinStateEnum};
use crate::level::Room;
use crate::{HEIGHT, WIDTH};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, Write, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ColliderList>,
    );

    fn run(&mut self, (mut tiles, players, input, list): Self::SystemData) {
        let mut collision_tiles = list.get();

        for (tile, player) in (&mut tiles, &players).join() {
            let mut proposed_tile = tile.clone();
            if input.action_is_down("up").unwrap_or(false) {
                proposed_tile.y -= 1;
            } else if input.action_is_down("down").unwrap_or(false) {
                proposed_tile.y += 1;
            } else if input.action_is_down("left").unwrap_or(false) {
                proposed_tile.x -= 1;
            } else if input.action_is_down("right").unwrap_or(false) {
                proposed_tile.x += 1;
            }

            let mut works = true;
            for possibility in &collision_tiles {
                if &proposed_tile == possibility {
                    works = false;
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
    }
}
