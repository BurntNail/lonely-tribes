use crate::components::{
    colliders::ColliderList,
    player::Player,
    tile_transform::TileTransform,
    win_state::{GameWinState, WinStateEnum},
};
use amethyst::ecs::{Join, Read, ReadStorage, System, Write};
use std::collections::HashMap;

///System for checking when the game is won
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
            //region add current player to list
            let id = player.id;
            let current_count = player_count.remove(&id).unwrap_or(0);
            player_count.insert(id, current_count + 1);
            //endregion

            //region check for whether or not the current player is on top of another player
            for (trigger_tile, trigger_type) in &trigger_tiles {
                let trigger_id = &trigger_type.get_id();
                if player_tile == trigger_tile {
                    if &id == trigger_id {
                        let match_no = count_match.remove(&id).unwrap_or(0);
                        count_match.insert(id, match_no + 1);
                    } else if trigger_id <= &3 {
                        //4 players, starting from ind 0
                        count_bad += 1;
                    }
                }
            }
            //endregion
        }

        if count_bad > 0 {
            gws.ws = WinStateEnum::End { won: false };
            return;
        }
        if count_match.is_empty() || player_count.is_empty() || count_match.len() != player_count.len(){
            return;
        }

        for (k, v) in &count_match {
            let pc = player_count.get(k).unwrap_or(&9999);
            let expected = pc * pc;

            if v != &expected {
                return;
            }
        }

        log::info!("WIN - Match: {:?}, PC: {:?}", count_match, player_count);
        gws.ws = WinStateEnum::End { won: true };
    }
}
