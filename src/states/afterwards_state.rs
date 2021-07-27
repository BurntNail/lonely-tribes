use crate::high_scores::HighScores;
use crate::{
    components::{GameWinState, WinStateEnum},
    states::{
        states_util::load_font,
        LEVELS,
    },
};
use amethyst::{core::ecs::{Builder, World, WorldExt}, input::VirtualKeyCode, ui::{Anchor, LineMode, UiText, UiTransform}, {GameData, SimpleState, SimpleTrans, StateData, StateEvent}, Trans};
use std::collections::HashMap;
use amethyst::input::InputEvent;
use crate::states::PuzzleState;

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
        log::info!("Score for {} = {}", level_from, score);
        let mut high_score = HighScores::default();
        let nu_high_score = high_score.add_score_and_write(level_from, score);

        let won_txt = if won {
            "You Won! Press [R] to Restart, or [N] to go to the Next Level" //Don't need to worry about winning - this will never happen because we will have the true end
        } else {
            "You Lost... Press [R] to Restart."
        };

        let won_txt = if nu_high_score.is_none() {
            format!("You got a new high score - {}! {}", score, won_txt)
        } else {
            format!(
                "You didn't beat your high score of {}... {}",
                nu_high_score.unwrap_or_else(|| unreachable!()),
                won_txt
            )
        };

        log::info!("{}", won_txt);

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
        75.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world.create_entity().with(trans).with(txt).build();
}
