use crate::{high_scores::HighScores, states::{
    game_state::{PuzzleState, LEVELS},
    states_util::{get_scaling_factor, load_font},
    welcome_state::StartGameState,
}, HOVER_COLOUR, Either};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use std::collections::HashMap;

pub struct LevelSelectState {
    buttons: HashMap<Entity, usize>,
    proc_gen: Option<Entity>,
    next_level: usize,
}

impl Default for LevelSelectState {
    fn default() -> Self {
        Self {
            buttons: HashMap::new(),
            proc_gen: None,
            next_level: 0,
        }
    }
}

impl SimpleState for LevelSelectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        let (buttons, next_level, proc_gen) = init_menu(world);
        self.buttons = buttons;
        self.next_level = next_level;
        self.proc_gen = Some(proc_gen);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;

        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => {
                use VirtualKeyCode::*;
                match key_code {
                    Return | Space => {
                        t = Trans::Switch(Box::new(PuzzleState::new(Either::One(self.next_level))))
                    }
                    Escape | Delete => t = Trans::Switch(Box::new(StartGameState::default())),
                    _ => {}
                }
            }
            StateEvent::Ui(event) => {
                let target_index = {
                    let mut index = None;

                    self.buttons.iter().for_each(|(entity, i)| {
                        if entity == &event.target {
                            index = Some(Either::One(*i));
                        }
                    });
                    if let Some(proc_gen) = self.proc_gen {
                        if proc_gen == event.target {
                            index = Some(Either::Two(100)) //TODO: Change this
                        }
                    }
                    index
                };

                if let Some(target_index) = target_index {
                    let mut texts = data.world.write_storage::<UiText>();
                    let txt = texts.get_mut(event.target);

                    if let Some(txt) = txt {
                        match event.event_type {
                            UiEventType::ClickStop => {
                                t = SimpleTrans::Switch(Box::new(PuzzleState::new(target_index)));
                            }
                            UiEventType::HoverStart => txt.color = HOVER_COLOUR,
                            UiEventType::HoverStop => txt.color = [1.0; 4],
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        t
    }
}

///Function to initialise the Level Select
///
/// Returns an Hashmap with the Entities to the indicies of level paths in *LEVELS*, as well as the next level to play, and a button for the proc-gen level
fn init_menu(world: &mut World) -> (HashMap<Entity, usize>, usize, Entity) {
    let sf = get_scaling_factor();
    let mut map = HashMap::new();
    let font_handle = load_font(world, "ZxSpectrum");
    let high_scores = HighScores::new();

    let level_txt_height = {
        let no_levels = LEVELS.len() as i32;
        let tot_height = (sf * 900.0) as i32;
        let buffer_space = (sf * 200.0) as i32;

        (tot_height - buffer_space) / no_levels
    };
    let get_height = |index: usize| {
        let pos = level_txt_height as f32 * (LEVELS.len() - index) as f32;
        pos - (sf * 450.0)
    };

    let main_trans = UiTransform::new(
        "help_main".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf * 350.0,
        0.5,
        sf * 1500.0,
        sf * 100.0,
    );
    let main_txt = UiText::new(
        load_font(world, "ZxSpectrumBold"),
        "Welcome to the Level Select. Press [Space] Or [Return] to Automatically go to the next unlocked level (or the last level if you have finished the game)".to_string(),
        [1.0; 4],
        sf * 33.3,
        LineMode::Wrap,
        Anchor::Middle
    );
    world
        .create_entity()
        .with(main_trans)
        .with(main_txt)
        .build();

    let next_level = high_scores.find_next_level();
    for (i, level) in LEVELS.iter().enumerate() {
        let high_score = high_scores.get_high_score(i);
        #[allow(clippy::collapsible_else_if)]
        let (text, colour, can_be_played) = if let Some(score) = high_score {
            (
                format!("Level number: {:02}, High Score of: {}", i + 1, score),
                [1.0; 4],
                true,
            )
        } else {
            if i == next_level {
                (format!("Level number: {:02}", i + 1), [1.0; 4], true)
            } else {
                (
                    format!("Level number: {:02}", i + 1),
                    [1.0, 0.25, 0.25, 1.0],
                    false,
                )
            }
        };

        let font_height = sf * 50.0;
        let trans = UiTransform::new(
            format!("{}-text", level),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            get_height(i), //already multiplied by sf in func
            0.5,
            sf * 1500.0,
            font_height,
        );
        let txt = UiText::new(
            font_handle.clone(),
            text,
            colour,
            font_height,
            LineMode::Wrap,
            Anchor::MiddleLeft,
        );

        if can_be_played {
            let ent = world
                .create_entity()
                .with(trans)
                .with(txt)
                .with(Interactable)
                .build();
            map.insert(ent, i);
        } else {
            world.create_entity().with(trans).with(txt).build();
        }
    }


    let proc_gen = {
        let font_height = sf * 50.0;
        let trans = UiTransform::new(
            "proc_gen_lvl".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            get_height(LEVELS.len()), //already multiplied by sf in func
            0.5,
            sf * 1500.0,
            font_height,
        );
        let txt = UiText::new(
            font_handle,
            "Procedural Generation!".to_string(),
            [1.0; 4],
            font_height,
            LineMode::Wrap,
            Anchor::MiddleLeft,
        );
        world.create_entity().with(trans).with(txt).with(Interactable).build()
    };


    (map, next_level, proc_gen)
}
