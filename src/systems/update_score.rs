use crate::components::{score::Score, win_state::GameState};
use amethyst::{
    core::ecs::{Join, Read, ReadStorage, System, WriteStorage},
    ui::UiText,
};
use crate::components::win_state::GameModeManager;

///System to update the score in the PuzzleState
pub struct ScoreUpdaterSystem;

impl<'s> System<'s> for ScoreUpdaterSystem {
    type SystemData = (
        Read<'s, GameState>,
        Read<'s, GameModeManager>,
        ReadStorage<'s, Score>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (gws, gmm, scores, mut texts): Self::SystemData) {
        let score = gws.level_no_of_moves;
        let moves_left = gmm.moves_left;

        for (_, text) in (&scores, &mut texts).join() {
            text.text = format!("Current Score: {}.\nSPECIAL moves left: {}", score, moves_left);
        }
    }
}
