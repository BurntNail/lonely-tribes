use crate::{
    high_scores::HighScores,
    states::{
        game_state::{PuzzleState, LEVELS},
        states_util::{get_scaling_factor, load_font},
        welcome_state::StartGameState,
    },
    Either, HOVER_COLOUR,
};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use rand::Rng;
use std::collections::HashMap;
use amethyst::core::Hidden;
use crate::states::states_util::load_sprite_sheet;
use amethyst::ui::UiImage;
use amethyst::renderer::SpriteRender;

pub struct LevelSelectState {
    buttons: HashMap<usize, HashMap<Entity, usize>>,
    proc_gen: Option<Entity>,
    left_btn: Option<Entity>,
    right_btn: Option<Entity>,
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
                            index = Some(Either::Two(rand::thread_rng().gen()))
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

pub const MAX_LEVELS_ONE_SCREEN: i32 = 6;

///Function to initialise the Level Select
///
/// Returns an Hashmap with the Entities to the indicies of level paths in *LEVELS*, as well as the next level to play, and a button for the proc-gen level, and the back and forward buttons
fn init_menu(world: &mut World) -> (Vec<HashMap<Entity, usize>>, usize, Entity, Entity, Entity) {
    let sf = get_scaling_factor();
    let mut map: Vec<HashMap<Entity, usize>> = HashMap::new();
    let font_handle = load_font(world, "ZxSpectrum");
    let high_scores = HighScores::new();

    let level_txt_height = {
        let tot_height = (sf * 900.0) as i32;
        let buffer_space = (sf * 200.0) as i32;

        (tot_height - buffer_space) / MAX_LEVELS_ONE_SCREEN
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
    let mut current_screen = 0;
    for (i, level) in LEVELS.iter().enumerate() {
        current_screen = MAX_LEVELS_ONE_SCREEN / i;

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

        let mut ent = world.create_entity().with(trans).with(txt);
        if can_be_played {
            ent = ent.with(Interactable);
        }
        if current_screen != 0 {
            ent = ent.with(Hidden);
        }

        let mut scrn = map.remove(&current_screen as usize).unwrap_or_default();
        scrn.insert(ent.build(), i);
        map[current_screen as usize] = scrn;
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
        let mut ent = world
            .create_entity()
            .with(trans)
            .with(txt)
            .with(Interactable);

        let current_scrn = MAX_LEVELS_ONE_SCREEN / LEVELS.len();

        if current_screen != 0 {
            ent = ent.with(Hidden);
        }

        ent.build()
    };

    let (left_btn, right_btn) = {
        let spritesheet = load_sprite_sheet(world, "left_right");

        let left = UiImage::Sprite(SpriteRender::new(spritesheet.clone(), 1));
        let right = UiImage::Sprite(SpriteRender::new(spritesheet, 2));

        let left_trans = UiTransform::new(
            "left_btn".to_string(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            sf * -900.0,
            sf * -450.0,
            0.5,
            sf * 27.0,
            sf * 27.0
        );

        let left_btn = world
            .create_entity()
            .with(left)
            .with(left_trans)
            .with(Interactable)
            .build();

        (left_btn, world.create_entity().build())
    };

    (map, next_level, proc_gen, left_btn, right_btn)
}
