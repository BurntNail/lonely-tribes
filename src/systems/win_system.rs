use crate::components::{ColliderList, GameWinState, Player, TileTransform, WinStateEnum};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write},
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
        let trigger_tiles = list.get_triggers();

        let mut count_match = HashMap::new();
        let mut count_bad = 0;
        let mut player_count = HashMap::new();

        for (player_tile, player) in (&tiles, &players).join() {
            let id = player.id;
            player_count.insert(id, player_count.get(&id).unwrap_or(&0) + 1);

            for (trigger_tile, trigger_id) in &trigger_tiles {
                let t_id = &trigger_id.get_id();
                if player_tile == trigger_tile {
                    if &id == t_id {
                        count_match.insert(id, count_match.get(&id).unwrap_or(&0) + 1);
                    } else if t_id <= &3 {
                        //4 players, starting from ind 0
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
            }
        }
        if win {
            gws.ws = WinStateEnum::End { won: true };
        }
    }
}
