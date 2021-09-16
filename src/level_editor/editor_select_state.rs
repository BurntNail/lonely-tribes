use crate::{
    level_editor::level_editor_state::LevelEditorState,
    states::{
        states_util::{get_scaling_factor, load_font},
        welcome_state::StartGameState,
    },
    HOVER_COLOUR,
};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};
use std::collections::HashMap;
use crate::states::game_state::get_levels_str;

#[derive(Default)]
pub struct LevelEditorLevelSelectState {
    buttons: HashMap<Option<usize>, Entity>,
}

impl SimpleState for LevelEditorLevelSelectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();
        self.buttons = init_menu(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;

        match event {
            StateEvent::Ui(event) => {
                let my_target = {
                    let mut target = None;
                    self.buttons.iter().for_each(|(index, ent)| {
                        if ent == &event.target {
                            target = Some((ent, index));
                        }
                    });
                    target
                };

                if let Some(targ) = my_target {
                    let mut texts = data.world.write_component::<UiText>();

                    if let Some(txt) = texts.get_mut(event.target) {
                        match event.event_type {
                            UiEventType::HoverStart => txt.color = HOVER_COLOUR,
                            UiEventType::HoverStop => txt.color = [1.0; 4],
                            UiEventType::ClickStop => {
                                t = SimpleTrans::Switch(Box::new(LevelEditorState::new(targ.1)))
                            }
                            _ => {}
                        }
                    }
                }
            }
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => {
                use VirtualKeyCode::*;
                match key_code {
                    Escape | Delete => t = SimpleTrans::Switch(Box::new(StartGameState::default())),
                    _ => {}
                };
            }
            _ => {}
        }

        t
    }
}

///Function to initialise the Level Editor Select
///
/// Returns an Hashmap with the Entities to the indicies of level paths in *LEVELS*, or in the case of None, a new level
pub fn init_menu(world: &mut World) -> HashMap<Option<usize>, Entity> {
    let sf = get_scaling_factor();
    let mut map = HashMap::new();
    let font_handle = load_font(world, "ZxSpectrum");
    let bold_handle = load_font(world, "ZxSpectrumBold");

    let lvls_len: i32 = get_levels_str().len() as i32;
    let level_txt_height = {
        let no_levels = lvls_len + 1;
        let tot_height = (sf * 900.0) as i32;
        let buffer_space = (sf * 200.0) as i32;

        (tot_height - buffer_space) / no_levels
    };
    let get_height = |index: usize, is_new: bool| {
        let pos = level_txt_height as f32
            * ((lvls_len - index as i32) + (if is_new { 0 } else { -1 })) as f32;
        pos - (450.0 * sf) + (100.0 * sf)
    };
    let he = |index: usize| get_height(index, false);

    let main_trans = UiTransform::new(
        "editor_select_main".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf * 350.0,
        0.5,
        sf * 1500.0,
        sf * 100.0,
    );
    let main_txt = UiText::new(
        bold_handle,
        "Welcome to the Level Editor Select. Click a level to edit it, or click New Level for a new level.".to_string(),
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

    let font_height = sf * 50.0;

    let new_trans = UiTransform::new(
        "new_level".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        get_height(0, true), //already sf-ed in func
        0.5,
        sf * 1500.0,
        font_height,
    );
    let new_txt = UiText::new(
        font_handle.clone(),
        "New Level".to_string(),
        [1.0; 4],
        font_height,
        LineMode::Single,
        Anchor::MiddleLeft,
    );
    map.insert(
        None,
        world
            .create_entity()
            .with(new_trans)
            .with(new_txt)
            .with(Interactable)
            .build(),
    );

    get_levels_str().into_iter().for_each(|name| {
        let level_no: usize = name
            .replace("lvl-", "")
            .replace(".png", "")
            .parse()
            .unwrap_or_else(|err| {
                log::error!("Error parsing level {} - {}", name, err);
                0
            })
            - 1;
        let text = format!("Edit Level Number: {:02}", level_no);

        let trans = UiTransform::new(
            text.clone(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            he(level_no),
            0.5,
            sf * 1500.0,
            font_height,
        );
        let txt = UiText::new(
            font_handle.clone(),
            text,
            [1.0; 4],
            font_height,
            LineMode::Single,
            Anchor::MiddleLeft,
        );

        map.insert(
            Some(level_no),
            world
                .create_entity()
                .with(trans)
                .with(txt)
                .with(Interactable)
                .build(),
        );
    });

    map
}
