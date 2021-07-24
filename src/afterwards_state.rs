use crate::components::{GameWinState, WinStateEnum};
use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::ecs::{Builder, Entity, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::window::ScreenDimensions;
use amethyst::{GameData, SimpleState, StateData, StateEvent, SimpleTrans, Trans};
use amethyst::input::{StringBindings, InputEvent, VirtualKeyCode};
use crate::game_state::PuzzleState;

pub struct PostGameState;

impl SimpleState for PostGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let won_txt = format!("{} Press [R] to Restart", get_win_txt(world));
        log::info!("{}", won_txt.clone());

        get_end_txt(world, won_txt);
        init_camera(world, (200.0, 60.0));
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => {
                match input_event {
                    InputEvent::KeyPressed { key_code, .. } => {
                        match key_code {
                            VirtualKeyCode::R => Trans::Switch(Box::new(PuzzleState::default())),
                            _ => Trans::None
                        }
                    }
                    _ => Trans::None
                }
            }
            _ => Trans::None
        }
    }
}

pub fn get_win_txt(world: &World) -> String {
    let gws = world.read_resource::<GameWinState>();

    let won = match gws.ws {
        WinStateEnum::End { won } => won,
        _ => false,
    };
    let won_txt = if won { "You Won!" } else { "You Lost..." };
    won_txt.to_string()
}

pub fn get_end_txt(world: &mut World, won_txt: String) {

    let font = world.read_resource::<Loader>().load(
        "fonts/ZxSpectrum.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let trans = UiTransform::new(
        "won_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.5,
        1000.0,
        1000.0,
    );
    let txt = UiText::new(
        font,
        won_txt,
        [1.0; 4],
        75.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world.create_entity().with(trans).with(txt).build();
}

fn init_camera(world: &mut World, wh: (f32, f32)) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(wh.0 * 0.5, wh.1 * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(wh.0, wh.1))
        .with(transform)
        .build();
}
