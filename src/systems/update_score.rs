use crate::components::{score::Score, win_state::GameWinState};
use amethyst::{
    core::ecs::{Join, Read, ReadStorage, System, WriteStorage},
    ui::UiText,
};

///System to update the score in the PuzzleState
pub struct ScoreUpdaterSystem;

impl<'s> System<'s> for ScoreUpdaterSystem {
    type SystemData = (
        Read<'s, GameWinState>,
        ReadStorage<'s, Score>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (gws, scores, mut texts): Self::SystemData) {
        let score = gws.level_no_of_moves;

        for (_, text) in (&scores, &mut texts).join() {
            text.text = format!("Current Score: {}", score);
        }
    }
}
