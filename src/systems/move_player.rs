use crate::components::{ColliderList, Player, TileTransform};
use crate::{HEIGHT, WIDTH};
use amethyst::core::Time;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ColliderList>,
        // Read<'s, Time>,
    );

    fn run(&mut self, (mut tiles, players, input, list): Self::SystemData) {
        let collision_tiles = list.get();

        for (tile, _) in (&mut tiles, &players).join() {
            let mut proposed_tile = *tile;

            if input.action_is_down("Up").unwrap_or(false) {
                proposed_tile.y -= 1;
            } else if input.action_is_down("Down").unwrap_or(false) {
                proposed_tile.y += 1;
            } else if input.action_is_down("Left").unwrap_or(false) {
                proposed_tile.x -= 1;
            } else if input.action_is_down("Right").unwrap_or(false) {
                proposed_tile.x += 1;
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
    }
}
