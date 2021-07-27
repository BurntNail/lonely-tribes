use crate::states::HelpState;
use crate::{
    components::TextWobble,
    states::{states_util::load_font, PuzzleState},
};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiImage, UiText, UiTransform},
    {GameData, SimpleState, SimpleTrans, StateData, StateEvent},
};

///State for welcoming the player to the game
#[derive(Default)]
pub struct StartGameState {
    ///Stores the Entity for the Button as an option for easier initialisation
    start_btn: Option<Entity>,
    help_btn: Option<Entity>,
}

impl SimpleState for StartGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        world.register::<Interactable>();
        world.register::<UiImage>();

        let (s, h) = init_menu(world);
        self.start_btn = Some(s);
        self.help_btn = Some(h);
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
                    let is_start = ui_event.target == start_btn;
                    let is_help = ui_event.target == help_btn;

                    if is_start || is_help {
                        let mut texts = data.world.write_storage::<UiText>();
                        let txt = texts.get_mut(ui_event.target);

                        if let Some(txt) = txt {
                            match ui_event.event_type {
                                UiEventType::ClickStop => {
                                    txt.color = [1.0, 1.0, 1.0, 0.5];
                                    if is_start {
                                        t = SimpleTrans::Switch(Box::new(PuzzleState::default()));
                                    } else if is_help {
                                        t = SimpleTrans::Switch(Box::new(HelpState::default()));
                                    }
                                }
                                UiEventType::HoverStart => txt.color = [1.0, 1.0, 1.0, 0.5],
                                UiEventType::HoverStop => txt.color = [1.0; 4],
                                _ => {}
                            };
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
/// Returns an Entity with the Start Button, and one with the Help Button
fn init_menu(world: &mut World) -> (Entity, Entity) {
    let bold_font_handle = load_font(world, "ZxSpectrumBold");
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
        String::from("Welcome to Making Friends!"),
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

    let font_handle = load_font(world, "ZxSpectrum");
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
        [1.0, 1.0, 1.0, 0.5],
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
        font_handle,
        String::from("Click here to get Help."),
        [1.0, 1.0, 1.0, 0.5],
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

    (start, help)
}
