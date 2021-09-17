use crate::{help_state::HelpState, level_select::LevelSelectState};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiImage, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};
use lonely_tribes_lib::{
    audio::init_audio,
    states_util::{get_scaling_factor, load_font},
    HOVER_COLOUR,
};
use lonely_tribes_components::text_wobble::TextWobble;

///State for welcoming the player to the game
#[derive(Default)]
pub struct StartGameState {
    ///Stores the Entity for the Start Button as an option for easier initialisation
    start_btn: Option<Entity>,
    ///Stores the Entity for the Help Button as an option for easier initialisation
    help_btn: Option<Entity>,
    ///Stores the Entity for the Quit as an option for easier initialisation
    quit_btn: Option<Entity>,
}

impl SimpleState for StartGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        world.register::<Interactable>();
        world.register::<UiImage>();

        init_audio(world);

        let (s, h, q) = init_menu(world);
        self.start_btn = Some(s);
        self.help_btn = Some(h);
        self.quit_btn = Some(q);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;

        match event {
            StateEvent::Ui(ui_event) => {
                if let Some(start_btn) = self.start_btn {
                    if let Some(help_btn) = self.help_btn {
                        if let Some(quit_btn) = self.quit_btn {
                            let is_start = ui_event.target == start_btn;
                            let is_help = ui_event.target == help_btn;
                            let is_quit = ui_event.target == quit_btn;

                            if is_start || is_help || is_quit {
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
                                                t = SimpleTrans::Switch(Box::new(
                                                    HelpState::default(),
                                                ));
                                            } else if is_quit {
                                                std::process::exit(0);
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
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => {
                if key_code == VirtualKeyCode::Space {
                    t = SimpleTrans::Switch(Box::new(LevelSelectState::default()));
                }
            }
            _ => {}
        }

        t
    }
}

///Function to initialise Start Screen Main Menu
///
/// Returns an Entity with the Start Button, one with the Help Button, one with the Level Editor button, and one with the Quit button
fn init_menu(world: &mut World) -> (Entity, Entity, Entity) {
    let sf = get_scaling_factor();
    let bold_font_handle = load_font(world, "ZxSpectrumBold");
    let font_handle = load_font(world, "ZxSpectrum");

    //region welcome
    let welcome_trans = UiTransform::new(
        String::from("welcome_txt"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf * 100.0,
        0.0,
        sf * 1000.0,
        sf * 250.0,
    );
    let welcome_txt = UiText::new(
        bold_font_handle,
        String::from("Welcome to Lonely Tribes!"),
        [1.0, 1.0, 1.0, 0.5],
        sf * 75.0,
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
        sf * -85.0,
        0.0,
        sf * 1000.0,
        sf * 40.0,
    );
    let start_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to Start."),
        [1.0; 4],
        sf * 50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let start = world
        .create_entity()
        .with(start_btn_trans)
        .with(start_btn_txt)
        .with(TextWobble::new(sf * 10.0, sf * -85.0, 2.5))
        .with(Interactable)
        .build();
    //endregion

    //region help
    let help_btn_trans = UiTransform::new(
        String::from("help_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf * -145.0,
        0.0,
        sf * 1000.0,
        sf * 40.0,
    );
    let help_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to get Help."),
        [1.0; 4],
        sf * 50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let help = world
        .create_entity()
        .with(help_btn_trans)
        .with(help_btn_txt)
        .with(TextWobble::new(sf * 10.0, sf * -145.0, 2.5))
        .with(Interactable)
        .build();
    //endregion

    //region quit
    let quit_btn_trans = UiTransform::new(
        String::from("quit_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf * -265.0,
        0.0,
        sf * 1500.0,
        sf * 40.0,
    );
    let quit_btn_text = UiText::new(
        font_handle,
        String::from("Exit Game"),
        [1.0; 4],
        sf * 50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let quit = world
        .create_entity()
        .with(quit_btn_trans)
        .with(quit_btn_text)
        .with(TextWobble::new(sf * 10.0, sf * -265.0, 2.5))
        .with(Interactable)
        .build();
    //endregion

    (start, help, quit)
}
