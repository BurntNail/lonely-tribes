use super::{help_state::HelpState, level_select::LevelSelectState};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiImage, UiText, UiTransform},
    winit::{Event, Window, WindowEvent},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};
use lonely_tribes_components::text_wobble::TextWobble;
use lonely_tribes_lib::{
    audio::init_audio,
    config::{change_screen, change_screen_res, DEFAULT_DPI, DEFAULT_SCREEN_RES},
    states_util::{get_scaling_factor, load_font},
    CONFIG, HOVER_COLOUR,
};
use lonely_tribes_systems::message_system::{MessageList, TimedMessagesToAdd};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use amethyst::core::Time;

pub static WELCOME_MSGS: &[&str] = &[
    "Hello!",
    "Join our discord!",
    "Email the creator!"
];
pub const SCALE_SPEED: f32 = 0.1;

///State for welcoming the player to the game
#[derive(Default)]
pub struct StartGameState {
    btns: HashMap<ButtonType, Entity>,
    welcome: Option<Entity>,
    time_elapsed: f32,
    total_time: f32
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ButtonType {
    Start,
    Help,
    Quit,
}

impl SimpleState for StartGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        world.register::<Interactable>();
        world.register::<UiImage>();
        world.insert(MessageList::default()); //no idea why I should do this, but otherwise it errors out
        world.insert(TimedMessagesToAdd::default());

        init_audio(world);

        if CONFIG.conf.screen_dimensions == DEFAULT_SCREEN_RES
            || CONFIG.conf.dpi_factor == DEFAULT_DPI
        {
            let w = world.read_resource::<Window>();
            let monitor = w.get_current_monitor();
            let res = monitor.get_dimensions();
            change_screen(
                res.width as u32,
                res.height as u32,
                monitor.get_hidpi_factor(),
            );
            std::process::exit(0);
        }

        self.btns = init_menu(world);
        self.welcome = Some(init_welcome_msgs(world));
        self.time_elapsed = 0.0;
        self.total_time = 2.0;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;

        match event {
            StateEvent::Window(Event::WindowEvent {
                window_id: _,
                event: WindowEvent::Resized(size),
            }) => change_screen_res(size.width as u32, size.height as u32),
            StateEvent::Ui(ui_event) => {
                let mut target = None;

                for (t, e) in &self.btns {
                    if &ui_event.target == e {
                        target = Some(*t);
                    }
                }

                if let Some(target) = target {
                    let mut texts = data.world.write_storage::<UiText>();
                    let txt = texts.get_mut(ui_event.target);

                    if let Some(txt) = txt {
                        match ui_event.event_type {
                            UiEventType::HoverStart => txt.color = HOVER_COLOUR,
                            UiEventType::HoverStop => txt.color = [1.0; 4],
                            UiEventType::ClickStart => txt.color = [1.0, 1.0, 1.0, 0.5],
                            UiEventType::ClickStop => {
                                match target {
                                    ButtonType::Start => {
                                        t = SimpleTrans::Switch(Box::new(
                                            LevelSelectState::default(),
                                        ));
                                    }
                                    ButtonType::Help => {
                                        t = SimpleTrans::Switch(Box::new(HelpState::default()));
                                    }
                                    ButtonType::Quit => {
                                        t = SimpleTrans::Quit;
                                    }
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        t
    }
    
    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = data.world;
        
        let te = world.read_resource::<Time>().delta_seconds() as f32;
        
        if let Some(welcome) = self.welcome {
            if let Some(txt) = world.write_component::<UiText>().get_mut(welcome) {
                self.time_elapsed += te;
                self.time_elapsed %= self.total_time;
    
                let sec_time = self.total_time / 4.0;
    
                let scale_mod = if self.time_elapsed < sec_time || self.time_elapsed > sec_time * 3.0 {
                    SCALE_SPEED
                } else {
                    -SCALE_SPEED
                };
        
                txt.font_size += scale_mod;
            }
        }
        
        
        
        SimpleTrans::None
    }
}

fn init_welcome_msgs (world: &mut World) -> Entity {
    let (sf_x, sf_y) = get_scaling_factor();
    let font_handle = load_font(world, "ZxSpectrum");
    let msg = WELCOME_MSGS[thread_rng().gen_range(0..WELCOME_MSGS.len())];
    
    let trans = UiTransform::new("welcome".to_string(), Anchor::Middle, Anchor::Middle, 400.0 * sf_x, 250.0 * sf_y, 0.0, sf_x * 500.0, sf_y * 500.0);
    let txt = UiText::new(font_handle, msg.to_string(), [1.0; 4], sf_y * 25.0, LineMode::Wrap, Anchor::Middle);
    
    world.create_entity().with(trans).with(txt).build()
}

///Function to initialise Start Screen Main Menu
///
/// Returns a hashmap of entities
fn init_menu(world: &mut World) -> HashMap<ButtonType, Entity> {
    let (sf_x, sf_y) = get_scaling_factor();
    let bold_font_handle = load_font(world, "ZxSpectrumBold");
    let font_handle = load_font(world, "ZxSpectrum");
    let mut map = HashMap::new();

    //region welcome
    let welcome_trans = UiTransform::new(
        String::from("welcome_txt"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf_y * 100.0,
        0.0,
        sf_x * 1000.0,
        sf_y * 250.0,
    );
    let welcome_txt = UiText::new(
        bold_font_handle,
        String::from("Welcome to Lonely Tribes!"),
        [1.0, 1.0, 1.0, 0.5],
        sf_y * 75.0,
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
        sf_y * -85.0,
        0.0,
        sf_x * 1000.0,
        sf_y * 40.0,
    );
    let start_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to Start."),
        [1.0; 4],
        sf_y * 50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    map.insert(
        ButtonType::Start,
        world
            .create_entity()
            .with(start_btn_trans)
            .with(start_btn_txt)
            .with(TextWobble::new(sf_y * 10.0, sf_y * -85.0, 2.5))
            .with(Interactable)
            .build(),
    );
    //endregion

    //region help
    let help_btn_trans = UiTransform::new(
        String::from("help_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf_y * -145.0,
        0.0,
        sf_x * 1000.0,
        sf_y * 40.0,
    );
    let help_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to get Help."),
        [1.0; 4],
        sf_y * 50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    map.insert(
        ButtonType::Help,
        world
            .create_entity()
            .with(help_btn_trans)
            .with(help_btn_txt)
            .with(TextWobble::new(sf_y * 10.0, sf_y * -145.0, 2.5))
            .with(Interactable)
            .build(),
    );
    //endregion

    //region quit
    let quit_btn_trans = UiTransform::new(
        String::from("quit_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        sf_y * -265.0,
        0.0,
        sf_x * 1500.0,
        sf_y * 40.0,
    );
    let quit_btn_text = UiText::new(
        font_handle,
        String::from("Exit Game"),
        [1.0; 4],
        sf_y * 50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    map.insert(
        ButtonType::Quit,
        world
            .create_entity()
            .with(quit_btn_trans)
            .with(quit_btn_text)
            .with(TextWobble::new(sf_y * 10.0, sf_y * -265.0, 2.5))
            .with(Interactable)
            .build(),
    );
    //endregion

    map
}
