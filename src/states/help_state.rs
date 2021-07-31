use crate::states::{states_util::load_font, StartGameState};
use amethyst::{
    core::ecs::{Builder, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, LineMode, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};

///Text displayed in HelpState
pub const HELP_TXT: &str = "Welcome to Making Friends!\n\nIn each level, there are different tribes of people who have gotten lost, and all the different tribes hate each other - so make all of the tribes meet back together without touching the other tribes!.\nUse WASD to move, Space to toggle showing the score, and R to restart if you get to a hard spot.\nPowerups - The guy who looks like he will trick you will randomly increase or decrease your score, the ghost/cross will kill half of all of the players (Thanos Snap), and the portal will teleport all of your players into random places (including walls and trees muah-ha-ha-ha-ha)\n\nHave fun!\n\n(Press Space or Return to get back to the main menu)";

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
