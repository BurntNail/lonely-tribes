use crate::{
    systems::{
    CollidersListSystem, EndOfGameSystem, FpsPrinterSystem, MovePlayerSystem, TextWobbleSystem,
    UpdateTileTransforms,
    },
};
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        palette::Srgba,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterSystem},
};
use structopt::StructOpt;
use log::LevelFilter;

#[macro_use]
extern crate lazy_static;

mod components;
mod level;
mod states;
mod systems;
mod tag;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 36;
pub const ARENA_WIDTH: u32 = 8 * WIDTH;
pub const ARENA_HEIGHT: u32 = 8 * HEIGHT; //each sprite is 8px wide, so arena will be 16 sprites by 9 sprites


fn main() -> amethyst::Result<()> {
    let opts = Flags::from_args();

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(app_root.join("config/bindings.ron"))?;

    let mut game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear(get_colours(34.0, 35.0, 35.0)),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(UpdateTileTransforms, "update_tile_transforms", &[])
        .with(CollidersListSystem, "collider_list", &[])
        .with(
            MovePlayerSystem::default(),
            "player_input",
            &["collider_list", "update_tile_transforms"],
        )
        .with(EndOfGameSystem, "end_of_game", &["collider_list"])
        .with(TextWobbleSystem, "txt_wobble", &[]);

    if !opts.console {
        log::set_max_level(LevelFilter::Error);
    } else if opts.fps {
        game_data = game_data
            .with(FpsCounterSystem, "fps", &[])
            .with(FpsPrinterSystem, "fps_printer", &["fps"]);
    }


    let mut game = Application::new(resources, states::StartGameState::default(), game_data)?;
    game.run();

    Ok(())
}

fn get_colours(r_a: f32, g_a: f32, b_a: f32) -> [f32; 4] {
    let (r, g, b, a) = Srgba::new(r_a / 255., g_a / 255., b_a / 255., 1.0)
        .into_linear()
        .into_components();
    [r, g, b, a]
}

#[derive(StructOpt, Debug)]
pub struct Flags {
    ///Enable an FPS counter in the console
    #[structopt(short, long)]
    pub fps: bool,

    ///Enable the console
    #[structopt(short, long)]
    pub console: bool
}

//todos

//TODO: Timer
//TODO: Save Score (time)

//TODO: dev cheats

//TODO: Power-Ups

//TODO: Level Select Screen
//TODO: Show level times

//TODO: Enemies
//TODO: Combat

//TODO: Music/SFX
