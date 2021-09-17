use amethyst::core::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use lonely_tribes_components::win_related::{GameState, GameModeManager};
use lonely_tribes_components::score::Score;
use amethyst::ui::UiText;

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
        let mode = gmm.current_mode;

        for (_, text) in (&scores, &mut texts).join() {
            text.text = format!(
                "Current Score: {}.\nSPECIAL moves left: {}.\nCurrent Mode: {:?}",
                score, moves_left, mode
            );
        }
    }
}
