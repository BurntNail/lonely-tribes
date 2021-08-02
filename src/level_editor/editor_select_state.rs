use crate::states::{game_state::LEVELS, states_util::load_font};
use amethyst::{core::ecs::{Builder, Entity, World, WorldExt}, ui::{Anchor, Interactable, LineMode, UiText, UiTransform}, GameData, SimpleState, StateData, StateEvent, SimpleTrans};
use std::collections::HashMap;
use amethyst::ui::UiEventType;
use crate::level_editor::level_editor_state::LevelEditorState;

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

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let mut t = SimpleTrans::None;

        if let StateEvent::Ui(event) = event {
            let my_target = {
                let mut target = None;
                self.buttons.iter().for_each(|(index, ent)| {
                    if ent == &event.target {
                        target = Some((ent, index));
                    }
                });
                target
            };

            let mut texts = data.world.write_component::<UiText>();

            if let Some(txt) = texts.get_mut(event.target) {
                match event.event_type {
                    UiEventType::HoverStart => txt.color = [1.0, 0.5, 0.75, 1.0],
                    UiEventType::HoverStop => txt.color = [1.0; 4],
                    UiEventType::ClickStop => {
                        if let Some(targ) = my_target {
                            t = SimpleTrans::Switch(Box::new(LevelEditorState::new(targ.1)));
                        }
                    }
                    _ => {}
                }
            }
        }

        t
    }
}

///Function to initialise the Level Editor Select
///
/// Returns an Hashmap with the Entities to the indicies of level paths in *LEVELS*, or in the case of None, a new level
pub fn init_menu(world: &mut World) -> HashMap<Option<usize>, Entity> {
    let mut map = HashMap::new();
    let font_handle = load_font(world, "ZxSpectrum");
    let bold_handle = load_font(world, "ZxSpectrumBold");

    let level_txt_height = {
        let no_levels = LEVELS.len() as i32 + 1;
        let tot_height = 900;
        let buffer_space = 200;

        (tot_height - buffer_space) / no_levels
    };
    let get_height = |index: usize, is_new: bool| {
        let pos = level_txt_height as f32
            * ((LEVELS.len() - index) as i32 + (if is_new { 0 } else { -1 })) as f32;
        pos - 450.0 + 100.0
    };
    let he = |index: usize| get_height(index, false);

    let main_trans = UiTransform::new(
        "editor_select_main".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        350.0,
        0.5,
        1500.0,
        100.0,
    );
    let main_txt = UiText::new(
        bold_handle,
        "Welcome to the Level Editor Select. Click a level to edit it, or click New Level for a new level.".to_string(),
        [1.0; 4],
        33.3,
        LineMode::Wrap,
        Anchor::Middle
    );
    world
        .create_entity()
        .with(main_trans)
        .with(main_txt)
        .build();

    let font_height = 50.0;

    let new_trans = UiTransform::new(
        "new_level".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        get_height(0, true),
        0.5,
        1500.0,
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

    LEVELS.clone().into_iter().for_each(|name| {
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
            1500.0,
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
