use crate::components::{
    Collider, ColliderList, GameWinState, Player, TileTransform, WinStateEnum,
};
use crate::level::Room;
use crate::{HEIGHT, WIDTH};
use amethyst::core::ecs::Entities;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
};
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct EndOfGameSystem;

impl<'s> System<'s> for EndOfGameSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, Player>,
        Read<'s, ColliderList>,
        Write<'s, GameWinState>,
    );

    fn run(&mut self, (tiles, players, list, mut gws): Self::SystemData) {
        let mut trigger_tiles = list.get_triggers();

        let mut count_match = HashMap::new();
        let mut count_bad = 0;
        let mut player_count = HashMap::new();

        for (player_tile, player) in (&tiles, &players).join() {
            let id = player.id;
            player_count.insert(id, player_count.get(&id).unwrap_or(&0) + 1);

            for (trigger_tile, trigger_id) in &trigger_tiles {
                if player_tile == trigger_tile {
                    if &id == trigger_id {
                        count_match.insert(id, count_match.get(&id).unwrap_or(&0) + 1);
                    } else {
                        count_bad += 1;
                    }
                }
            }
        }

        if count_bad > 0 {
            gws.ws = WinStateEnum::End { won: false };
            return;
        }

        let mut win = true;
        for (k, v) in &count_match {
            let pc = player_count.get(&k).unwrap_or(&9999);
            let expected = pc * pc;

            if v != &expected {
                win = false;
                // log::info!("Got {}, Expected {}", v, expected);
            }
        }
        if win {
            gws.ws = WinStateEnum::End { won: true };
        }
    }
}
