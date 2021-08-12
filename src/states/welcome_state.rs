use crate::{
    components::text_wobble::TextWobble,
    level_editor::editor_select_state::LevelEditorLevelSelectState,
    states::{help_state::HelpState, level_select::LevelSelectState, states_util::load_font},
    HOVER_COLOUR,
};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiImage, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};

///State for welcoming the player to the game
#[derive(Default)]
pub struct StartGameState {
    ///Stores the Entity for the Start Button as an option for easier initialisation
    start_btn: Option<Entity>,
    ///Stores the Entity for the Help Button as an option for easier initialisation
    help_btn: Option<Entity>,
    ///Stores the Entity for the Level Editor Button as an option for easier initialisation
    level_btn: Option<Entity>,
}

impl SimpleState for StartGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        world.register::<Interactable>();
        world.register::<UiImage>();

        let (s, h, l) = init_menu(world);
        self.start_btn = Some(s);
        self.help_btn = Some(h);
        self.level_btn = Some(l);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;

        if let StateEvent::Ui(ui_event) = event {
            if let Some(start_btn) = self.start_btn {
                if let Some(help_btn) = self.help_btn {
                    if let Some(lvl_btn) = self.level_btn {
                        let is_start = ui_event.target == start_btn;
                        let is_help = ui_event.target == help_btn;
                        let is_level = ui_event.target == lvl_btn;

                        if is_start || is_help || is_level {
                            let mut texts = data.world.write_storage::<UiText>();
                            let txt = texts.get_mut(ui_event.target);

                            if let Some(txt) = txt {
                                match ui_event.event_type {
                                    UiEventType::ClickStop => {
                                        txt.color = [1.0, 1.0, 1.0, 0.5];
                                        if is_start {
                                            t = SimpleTrans::Switch(Box::new(
                                                LevelSelectState::default(),
                                            ));
                                        } else if is_help {
                                            t = SimpleTrans::Switch(Box::new(HelpState::default()));
                                        } else if is_level {
                                            t = SimpleTrans::Switch(Box::new(
                                                LevelEditorLevelSelectState::default(),
                                            ));
                                        }
                                    }
                                    UiEventType::HoverStart => txt.color = HOVER_COLOUR,
                                    UiEventType::HoverStop => txt.color = [1.0; 4],
                                    _ => {}
                                };
                            }
                        }
                    }
                }
            }
        }

        t
    }
}

///Function to initialise Start Screen Main Menu
///
/// Returns an Entity with the Start Button, one with the Help Button, and one with the Level Editor button
fn init_menu(world: &mut World) -> (Entity, Entity, Entity) {
    let bold_font_handle = load_font(world, "ZxSpectrumBold");
    let font_handle = load_font(world, "ZxSpectrum");

    //region welcome
    let welcome_trans = UiTransform::new(
        String::from("welcome_txt"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        100.0,
        0.0,
        1000.0,
        250.0,
    );
    let welcome_txt = UiText::new(
        bold_font_handle,
        String::from("Welcome to Lonely Tribes!"),
        [1.0, 1.0, 1.0, 0.5],
        75.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world
        .create_entity()
        .with(welcome_trans)
        .with(welcome_txt)
        .build();
    //endregion

    //region start
    let start_btn_trans = UiTransform::new(
        String::from("start_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        -85.0,
        0.0,
        1000.0,
        40.0,
    );
    let start_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to Start."),
        [1.0; 4],
        50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let start = world
        .create_entity()
        .with(start_btn_trans)
        .with(start_btn_txt)
        .with(TextWobble::new(10.0, -85.0, 2.5))
        .with(Interactable)
        .build();
    //endregion

    //region help
    let help_btn_trans = UiTransform::new(
        String::from("help_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        -145.0,
        0.0,
        1000.0,
        40.0,
    );
    let help_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to get Help."),
        [1.0; 4],
        50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let help = world
        .create_entity()
        .with(help_btn_trans)
        .with(help_btn_txt)
        .with(TextWobble::new(10.0, -145.0, 2.5))
        .with(Interactable)
        .build();
    //endregion

    //region level editor
    let editor_btn_trans = UiTransform::new(
        String::from("level_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        -205.0,
        0.0,
        1500.0,
        40.0,
    );
    let editor_btn_txt = UiText::new(
        font_handle,
        String::from("Click here to open the Level Editor."),
        [1.0; 4],
        50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let editor = world
        .create_entity()
        .with(editor_btn_trans)
        .with(editor_btn_txt)
        .with(TextWobble::new(10.0, -205.0, 2.5))
        .with(Interactable)
        .build();
    //endregion

    (start, help, editor)
}
