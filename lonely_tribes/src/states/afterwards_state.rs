use super::{game_state::PuzzleState, level_select::LevelSelectState};
use amethyst::{
    core::ecs::{Builder, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, LineMode, UiText, UiTransform},
    winit::{Event, WindowEvent},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use lonely_tribes_components::win_related::{GameState, GameStateEnum};
use lonely_tribes_generation::level::Level;
use lonely_tribes_lib::{
    config::change_screen_res,
    either::Either,
    high_scores::HighScores,
    states_util::{get_scaling_factor, levels_len, load_font},
    CONFIG,
};
use std::collections::HashMap;

///State for when after a *PuzzleState*
#[derive(Default)]
pub struct PostGameState {
    ///A HashMap containing key presses, which lead to indicies for levels in *LEVELS*
    map: HashMap<VirtualKeyCode, String>,
}

impl PostGameState {
    ///Constructor for PostGameState
    /// Initialises the Actions Map as an empty HashMap
    pub fn new() -> Self {
        Self::default()
    }
}

impl SimpleState for PostGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let (level_from, is_last_level, won, score) = get_stuff(world);
        let mut high_score = HighScores::new();

        let opts = CONFIG.flags;

        let mut nu_high_score = None;

        if !opts.debug && won && level_from.contains("lvl-") {
            nu_high_score = Some(high_score.add_score_and_write(level_from.clone(), score));
        }

        let won_txt = if won && level_from.contains("lvl-") {
            let win = "You Won! Press [R] to Restart, [N] to go to the Next Level, or [L] to go to Level Select.";
            if let Some(nu_high_score) = nu_high_score {
                if let Some(nu_high_score) = nu_high_score {
                    if nu_high_score == score {
                        format!(
                            "You got the same as your high score: {}...\n\n{}",
                            score, win
                        )
                    } else {
                        format!(
                            "You didn't beat your high score of {}, you only got: {}...\n\n{}",
                            nu_high_score, score, win
                        )
                    }
                } else {
                    format!("You got a new high score - {}!\n\n{}", score, win)
                }
            } else {
                format!(
                    "Debug Options are enabled, so High Scores are disabled, but...\n\n{}",
                    win
                )
            }
        } else if won {
            let seed = match Level::get_seed_index_from_path(&level_from) {
                Either::One(_) => "Error getting seed...".to_string(),
                Either::Two(s) => format!("{}", s),
            };
            format!(
                "Congrats on beating this procedurally generated level!\n You got {} on Seed: {}",
                score, seed
            )
        } else {
            "You Lost... Press [R] to Restart.".to_string()
        };

        let mut map = HashMap::new();
        map.insert(VirtualKeyCode::R, level_from.clone());
        if won && !is_last_level {
            if let Either::One(level_from) = Level::get_seed_index_from_path(&level_from) {
                map.insert(VirtualKeyCode::N, format!("lvl-{:02}.ron", level_from + 2));
            }
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

        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => {
                self.map.iter().for_each(|(k, v)| {
                    if &key_code == k {
                        t = Trans::Switch(Box::new(PuzzleState::new(v.clone())));
                    }
                });
                if key_code == VirtualKeyCode::L {
                    t = Trans::Switch(Box::new(LevelSelectState::default()));
                }
            }
            StateEvent::Window(Event::WindowEvent {
                window_id: _,
                event: WindowEvent::Resized(size),
            }) => change_screen_res(size.width as u32, size.height as u32),
            _ => {}
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
pub fn get_stuff(world: &World) -> (String, bool, bool, i32) {
    let gws = world.read_resource::<GameState>();

    let level_from = gws.level_from.clone();
    let is_last_level =
        if let Either::One(level_from) = Level::get_seed_index_from_path(&level_from) {
            level_from >= levels_len() - 1
        } else {
            false
        };
    let won = match gws.ws {
        GameStateEnum::End { lost_position } => lost_position.is_none(),
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
    let (sf_x, sf_y) = get_scaling_factor();
    let trans = UiTransform::new(
        "won_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.5,
        sf_x * 1000.0,
        sf_y * 1000.0,
    );
    let txt = UiText::new(
        load_font(world, "ZxSpectrum"),
        won_txt,
        [1.0; 4],
        sf_y * 50.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world.create_entity().with(trans).with(txt).build();
}
