use super::{game_state::PuzzleState, welcome_state::StartGameState};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    renderer::SpriteRender,
    ui::{Anchor, Interactable, LineMode, UiEventType, UiImage, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use lonely_tribes_lib::{
    either::Either,
    high_scores::HighScores,
    paths::get_directory,
    states_util::{
        get_levels, get_scaling_factor, levels_len, load_font, load_sprite_sheet, LevelType,
    },
    HOVER_COLOUR,
};
use rand::Rng;
use std::{collections::HashMap, fs::read_to_string};

pub struct LevelSelectState {
    buttons: HashMap<Entity, usize>,
    proc_gen: Option<Entity>,
    leftright: Option<(Entity, Entity)>,
    next_level: usize,
    current_screen: usize,
}

impl Default for LevelSelectState {
    fn default() -> Self {
        Self {
            buttons: HashMap::new(),
            proc_gen: None,
            next_level: 0,
            leftright: None,
            current_screen: 0,
        }
    }
}

impl SimpleState for LevelSelectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        let (buttons, next_level, proc_gen, lr) = create_lvl_select_btns(world, 0);
        self.buttons = buttons;
        self.next_level = next_level;
        self.proc_gen = Some(proc_gen);
        self.leftright = Some(lr);
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

                    {
                        let ints = data.world.read_storage::<Interactable>();
                        self.buttons.iter().for_each(|(entity, i)| {
                            if entity == &event.target && ints.contains(*entity) {
                                let ind = *i;
                                if ind > 1000 {
                                    index = Some(Either::Two((ind - 1000) as u32))
                                } else {
                                    index = Some(Either::One(ind));
                                }
                            }
                        });
                    }
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

                let is_left = {
                    let mut res = None;
                    if let Some((l, r)) = self.leftright {
                        if event.target == l {
                            res = Some(true);
                        } else if event.target == r {
                            res = Some(false);
                        }
                    }

                    res
                };
                let mut needs_to_redo_btns = false;
                if let Some(is_left) = is_left {
                    if data
                        .world
                        .read_storage::<UiImage>()
                        .get(event.target)
                        .is_some()
                        && event.event_type == UiEventType::ClickStop
                    {
                        if is_left {
                            //we need to go back one screen
                            if self.current_screen != 0 {
                                self.current_screen -= 1;
                                needs_to_redo_btns = true;
                            }
                        } else {
                            self.current_screen += 1;
                            needs_to_redo_btns = true;
                        }
                    }
                }

                if needs_to_redo_btns {
                    let (buttons, next_level, proc_gen, lr) =
                        create_lvl_select_btns(data.world, self.current_screen);
                    self.buttons = buttons;
                    self.next_level = next_level;
                    self.proc_gen = Some(proc_gen);
                    self.leftright = Some(lr);
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
fn create_lvl_select_btns(
    world: &mut World,
    current_screen: usize,
) -> (HashMap<Entity, usize>, usize, Entity, (Entity, Entity)) {
    let (sf_x, sf_y) = get_scaling_factor();
    world.delete_all();

    let mut map: HashMap<Entity, usize> = HashMap::new();
    let font_handle = load_font(world, "ZxSpectrum");
    let high_scores = HighScores::new();

    let level_txt_height = {
        let tot_height = (sf_y * 900.0) as i32;
        let buffer_space = (sf_y * 200.0) as i32;

        (tot_height - buffer_space) / MAX_LEVELS_ONE_SCREEN
    };
    let get_height = |index: usize| {
        let pos = level_txt_height as f32 * (MAX_LEVELS_ONE_SCREEN as usize - index) as f32;
        pos - (sf_y * 450.0)
    };

    let main_trans = UiTransform::new(
        "help_main".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf_y * 425.0,
        0.5,
        sf_x * 1500.0,
        sf_y * 100.0,
    );
    let main_txt = UiText::new(
        load_font(world, "ZxSpectrumBold"),
        "Welcome to the Level Select. Press [Space] Or [Return] to Automatically go to the next unlocked level (or the last level if you have finished the game)".to_string(),
        [1.0; 4],
        sf_y * 33.3,
        LineMode::Wrap,
        Anchor::Middle
    );
    world
        .create_entity()
        .with(main_trans)
        .with(main_txt)
        .build();

    let next_level = high_scores.find_next_level();
    for (i, (level, level_type)) in get_levels()
        .iter()
        .skip(current_screen * (MAX_LEVELS_ONE_SCREEN - 1) as usize)
        .take(5)
        .enumerate()
    {
        let i_adj = (current_screen as i32) * MAX_LEVELS_ONE_SCREEN + i as i32;

        let (text, colour, can_be_played, pg_ind) = {
            if level_type == &LevelType::Developer {
                let high_score = high_scores.get_high_score(i_adj as usize);

                #[allow(clippy::collapsible_else_if)]
                if let Some(score) = high_score {
                    (
                        format!("Level number: {:02}, High Score of: {}", i_adj + 1, score),
                        [1.0; 4],
                        true,
                        None,
                    )
                } else {
                    if i == next_level {
                        (
                            format!("Level number: {:02}", i_adj + 1),
                            [1.0; 4],
                            true,
                            None,
                        )
                    } else {
                        (
                            format!("Level number: {:02}", i_adj + 1),
                            [1.0, 0.25, 0.25, 1.0],
                            false,
                            None,
                        )
                    }
                }
            } else {
                if level_type == &LevelType::ProcGen {
                    let index = read_to_string(get_directory(false).join("../maps/").join(level))
                        .unwrap_or_default()
                        .parse::<usize>()
                        .unwrap_or_default();

                    (
                        format!(
                            "Procedurally Generated Level: {}",
                            level.replace("pg-", "").replace(".txt", "")
                        ),
                        [1.0; 4],
                        true,
                        Some(index),
                    )
                } else {
                    (
                        format!(
                            "User Created/Downloaded Level: {}",
                            level.replace("m-user-", "").replace(".png", "")
                        ),
                        [1.0; 4],
                        true,
                        None,
                    )
                }
            }
        };

        let font_height = sf_y * 50.0;
        let trans = UiTransform::new(
            format!("{}-text", level),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            get_height(i), //already multiplied by sf in func
            0.5,
            sf_x * 1500.0,
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

        let mut entity = world.create_entity().with(trans).with(txt);
        if can_be_played {
            entity = entity.with(Interactable);
        }

        map.insert(
            entity.build(),
            if level_type != &LevelType::ProcGen {
                i_adj as usize
            } else {
                1000 + pg_ind.unwrap_or(0)
            },
        );
    }

    let proc_gen = {
        let font_height = sf_y * 50.0;
        let trans = UiTransform::new(
            "proc_gen_lvl".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            get_height(MAX_LEVELS_ONE_SCREEN as usize - 1), //already multiplied by sf in func
            0.5,
            sf_x * 1500.0,
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
        world
            .create_entity()
            .with(trans)
            .with(txt)
            .with(Interactable)
            .build()
    };

    let lr = {
        let spritesheet = load_sprite_sheet(world, "left_right");

        let right_btn = if current_screen < (levels_len() / MAX_LEVELS_ONE_SCREEN as usize) {
            let right = UiImage::Sprite(SpriteRender::new(spritesheet.clone(), 1));
            let right_trans = UiTransform::new(
                "right_scrn_btn".to_string(),
                Anchor::BottomRight,
                Anchor::BottomRight,
                sf_x * -10.0,
                sf_y * 10.0,
                0.5,
                sf_x * 90.0,
                sf_x * 90.0, //for Sprites, they need to NOT scale in 2 dimensions
            );

            world
                .create_entity()
                .with(right)
                .with(right_trans)
                .with(Interactable)
                .build()
        } else {
            world.create_entity().build()
        };
        let left_btn = if current_screen > 0 {
            let left = UiImage::Sprite(SpriteRender::new(spritesheet, 0));

            let left_trans = UiTransform::new(
                "left_scrn_btn".to_string(),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                sf_x * 10.0,
                sf_y * 10.0,
                0.5,
                sf_x * 90.0,
                sf_x * 90.0, //for Sprites, they need to NOT scale in 2 dimensions
            );

            world
                .create_entity()
                .with(left)
                .with(left_trans)
                .with(Interactable)
                .build()
        } else {
            world.create_entity().build()
        };

        (left_btn, right_btn)
    };

    (map, next_level, proc_gen, lr)
}
