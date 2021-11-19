use super::welcome_state::StartGameState;
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiText, UiTransform},
    winit::{Event, WindowEvent},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use lonely_tribes_lib::{
    config::change_screen_res,
    states_util::{get_scaling_factor, load_font},
};

///State for when the user has finished all levels
#[derive(Default)]
pub struct TrueEnd {
    ///Stores the Entity for the Button as an option for easier initialisation
    btn: Option<Entity>,
}

impl SimpleState for TrueEnd {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.btn = Some(get_true_end_txt(world));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut back_to_mm = false;
        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => {
                if key_code == VirtualKeyCode::Return || key_code == VirtualKeyCode::Space {
                    back_to_mm = true;
                }
            }
            StateEvent::Ui(event) => {
                let is_target = if let Some(my_target) = self.btn {
                    event.target == my_target
                } else {
                    false
                };

                let mut txts = data.world.write_storage::<UiText>();
                let txt = txts.get_mut(event.target);

                if let Some(txt) = txt {
                    match event.event_type {
                        UiEventType::ClickStart => txt.color = [0.8, 0.8, 0.9, 1.0],
                        UiEventType::ClickStop => {
                            if is_target {
                                txt.color = [1.0, 1.0, 1.0, 0.5];
                                back_to_mm = true;
                            }
                        }
                        UiEventType::HoverStart => txt.color = [1.0, 1.0, 1.0, 0.5],
                        UiEventType::HoverStop => txt.color = [1.0; 4],
                        _ => {}
                    }
                }
            }
            StateEvent::Window(Event::WindowEvent {
                window_id: _,
                event: WindowEvent::Resized(size),
            }) => change_screen_res(size.width as u32, size.height as u32),
            _ => {}
        }

        if back_to_mm {
            Trans::Switch(Box::new(StartGameState::default()))
        } else {
            Trans::None
        }
    }
}

///Instantiates text with end text detailing how to get back to the main menu
///
///Returns the entity of that text, for checking when it was clicked
pub fn get_true_end_txt(world: &mut World) -> Entity {
    let (sf_x, sf_y) = get_scaling_factor();
    let trans = UiTransform::new(
        "end_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.5,
        sf_x * 1000.0,
        sf_y * 1000.0,
    );
    let txt = UiText::new(
        load_font(world, "ZxSpectrum"),
        "Congratulations on finishing Lonely Tribes! Now, a world of auto-gen levels awaits.... Click here, or press [Space] or [Enter] to go back to the Main Menu. Congrats!".to_string(),
        [1.0; 4],
        sf_y * 45.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world
        .create_entity()
        .with(trans)
        .with(txt)
        .with(Interactable)
        .build()
}
