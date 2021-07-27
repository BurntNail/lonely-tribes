use crate::states::states_util::load_font;
use crate::states::StartGameState;
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::input::{InputEvent, VirtualKeyCode};
use amethyst::ui::{Anchor, LineMode, UiText, UiTransform};
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent};

///Text displayed in HelpState
pub const HELP_TXT: &str = "Welcome to Making Friends!\n\nThe aim of the game is to get all the people to meet, but different types shouldn't touch.\nUse WASD to move, Space to toggle showing the score, and R to restart if you get to a hard spot.\n\nHave fun!\n\n(Press Space or Return to get back to the main menu)";

///State to show Help
#[derive(Default)]
pub struct HelpState;

impl SimpleState for HelpState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        get_help_txt(world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;
        if let StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) = event {
            if key_code == VirtualKeyCode::Space || key_code == VirtualKeyCode::Return {
                t = SimpleTrans::Switch(Box::new(StartGameState::default()));
            }
        }
        t
    }
}

///Function to insert text onto the Help screen
///The text is **not** interactable.
///
///By default, it uses a non-bold sans-serif font called ZxSpectrum
fn get_help_txt(world: &mut World) {
    let trans = UiTransform::new(
        "help_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.5,
        1500.0,
        900.0,
    );
    let txt = UiText::new(
        load_font(world, "ZxSpectrum"),
        HELP_TXT.to_string(),
        [1.0; 4],
        25.0,
        LineMode::Wrap,
        Anchor::MiddleLeft,
    );
    world.create_entity().with(trans).with(txt).build();
}