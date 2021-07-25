use crate::components::TextWobble;
use crate::states::states_util::load_font;
use amethyst::core::ecs::{Builder, World, WorldExt, Entity};
use amethyst::ui::{Anchor, LineMode, UiText, UiTransform, Interactable, UiEventType};
use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans};
use crate::states::PuzzleState;

#[derive(Default)]
pub struct StartGameState {
    btn: Option<Entity>
}

impl SimpleState for StartGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.btn = Some(init_menu(world));
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = event {
            let is_target = ui_event.target == self.btn.unwrap(); //TODO: Better solution than unwrap
            let mut texts = data.world.write_storage::<UiText>();
            let txt = texts.get_mut(ui_event.target);


            if let Some(txt) = txt {
                match ui_event.event_type {
                    UiEventType::ClickStart => {
                        txt.color = [0.8, 0.8, 0.9, 1.0];
                        SimpleTrans::None
                    }
                    UiEventType::ClickStop => {
                        if is_target {
                            txt.color = [1.0, 1.0, 1.0, 0.5];
                            SimpleTrans::Switch(Box::new(PuzzleState::default()))
                        } else {
                            SimpleTrans::None
                        }
                    }
                    UiEventType::HoverStart => {
                        txt.color = [1.0, 1.0, 1.0, 0.5];
                        SimpleTrans::None
                    }
                    UiEventType::HoverStop => {
                        txt.color = [1.0; 4];
                        SimpleTrans::None
                    }
                    _ => SimpleTrans::None
                }
            } else {
                SimpleTrans::None
            }
        } else {
            SimpleTrans::None
        }
    }
}

///Function to initialise Start Screen Main Menu
///
///  - Takes in the data.world
///
/// Returns an Entity with the Start Button
fn init_menu(world: &mut World) -> Entity {
    let font_handle = load_font(world, "ZxSpectrum");
    let welcome_trans = UiTransform::new(
        String::from("welcome_txt"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.0,
        1800.0,
        50.0,
    );
    let welcome_txt = UiText::new(
        font_handle.clone(),
        String::from("Welcome to Making Friends!"),
        [1.0, 1.0, 1.0, 0.5],
        75.0,
        LineMode::Single,
        Anchor::Middle,
    );

    world
        .create_entity()
        .with(welcome_trans)
        .with(welcome_txt)
        .build();

    let btn_trans = UiTransform::new(
        String::from("start_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        -85.0,
        0.0,
        750.0,
        30.0,
    );
    let btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to Start"),
        [1.0, 1.0, 1.0, 0.5],
        50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    world
        .create_entity()
        .with(btn_trans)
        .with(btn_txt)
        .with(TextWobble::new(10.0, -85.0, 2.5))
        .with(Interactable)
        .build()

}
