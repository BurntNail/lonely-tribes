use crate::{
    components::{GameWinState, WinStateEnum},
    high_scores::HighScores,
    states::{states_util::load_font, PuzzleState, LEVELS},
    Flags,
};
use amethyst::{
    core::ecs::{Builder, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, LineMode, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use std::collections::HashMap;
use structopt::StructOpt;

///State for when after a *PuzzleState*
pub struct PostGameState {
    ///A HashMap containing key presses, which lead to indicies for levels in *LEVELS*
    map: HashMap<VirtualKeyCode, usize>,
}

impl PostGameState {
    ///Constructor for PostGameState
    /// Initialises the Actions Map as an empty HashMap
    pub fn new() -> Self {
        PostGameState {
            map: HashMap::new(),
        }
    }
}

impl SimpleState for PostGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let (level_from, is_last_level, won, score) = get_stuff(world);
        let mut high_score = HighScores::default();

        let opts: Flags = Flags::from_args();

        let mut nu_high_score = None;

        if !opts.debug && won {
            nu_high_score = Some(high_score.add_score_and_write(level_from, score));
        }

        let won_txt = if won {
            let win = "You Won! Press [R] to Restart, or [N] to go to the Next Level";
            if let Some(nu_high_score) = nu_high_score {
                if nu_high_score.is_none() {
                    format!("You got a new high score - {}!\n\n{}", score, win)
                } else {
                    format!(
                        "You didn't beat your high score of {}...\n\n{}",
                        nu_high_score.unwrap_or_else(|| unreachable!()),
                        win
                    )
                }
            } else {
                format!(
                    "Debug Options are enabled, so High Scores are disabled, but...\n\n{}",
                    win
                )
            }
        } else {
            "You Lost... Press [R] to Restart.".to_string()
        };

        let mut map = HashMap::new();
        map.insert(VirtualKeyCode::R, level_from);
        if won && !is_last_level {
            map.insert(VirtualKeyCode::N, level_from + 1);
        }
        self.map = map;

        get_end_txt(world, won_txt);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = Trans::None;
        if let StateEvent::Input(event) = event {
            if let InputEvent::KeyPressed { key_code, .. } = event {
                self.map.iter().for_each(|(k, v)| {
                    if &key_code == k {
                        t = Trans::Switch(Box::new(PuzzleState::new(*v)));
                    }
                });
            }
        }
        t
    }
}

///Function to get necessary things for the PostGameState
///
/// Returns:
///  - A usize - the level before the PGS
///  - A bool - whether or not that was the last level
///  - Another bool - whether or not the previous level was won
///  - An f32 - the score from the previous level
pub fn get_stuff(world: &World) -> (usize, bool, bool, i32) {
    let gws = world.read_resource::<GameWinState>();

    let level_from = gws.level_from;
    let is_last_level = level_from >= LEVELS.len() - 1;
    let won = match gws.ws {
        WinStateEnum::End { won } => won,
        _ => false,
    };
    let score = gws.level_no_of_moves;

    (level_from, is_last_level, won, score)
}

///Function to insert text onto the PostGameState screen, with the win_text being that text
///The text is **not** interactable.
///
///By default, it uses a non-bold sans-serif font called ZxSpectrum
pub fn get_end_txt(world: &mut World, won_txt: String) {
    let trans = UiTransform::new(
        "won_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.5,
        1000.0,
        1000.0,
    );
    let txt = UiText::new(
        load_font(world, "ZxSpectrum"),
        won_txt,
        [1.0; 4],
        50.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world.create_entity().with(trans).with(txt).build();
}
