use crate::components::{
    player::Player,
    tile_transform::TileTransform,
    win_state::{GameState, GameStateEnum},
};
use amethyst::ecs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

///System for checking when the game is won
pub struct EndOfGameSystem;

impl<'s> System<'s> for EndOfGameSystem {
    type SystemData = (
		ReadStorage<'s, TileTransform>,
		ReadStorage<'s, Player>,
		Write<'s, GameState>,
    );

    fn run(&mut self, (tiles, players, mut gws): Self::SystemData) {
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
            for (check_tile, check_player) in (&tiles, &players).join() {
                if player_tile == check_tile {
                    if id == check_player.id {
                        let match_no = count_match.remove(&id).unwrap_or(0);
                        count_match.insert(id, match_no + 1);
                    } else {
                        count_bad += 1;
                    }
                }
            }
            //endregion
        }

        //region check for a loss or tbd
        if count_bad > 0 {
            gws.ws = GameStateEnum::End { won: false };
            return;
        }
        if count_match.is_empty()
            || player_count.is_empty()
            || count_match.len() != player_count.len()
        {
            return;
        }
        //endregion

        for (k, v) in &count_match {
            let pc = player_count.get(k).unwrap_or(&9999);
            let expected = pc * pc;

            if v != &expected {
                return;
            }
        }

        gws.ws = GameStateEnum::End { won: true };
    }
}
