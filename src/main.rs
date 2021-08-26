use crate::{
    high_scores::DATA_DIR,
    states::{help_state::HelpState, welcome_state::StartGameState},
    systems::{
        colliders_list_system::ListSystem, fps_counter::FpsPrinterSystem,
        mode_tinter_system::GameModeTinterSystem, move_player::MovePlayerSystem,
        txt_wobble_system::TextWobbleSystem, update_score::ScoreUpdaterSystem,
        update_tile_transforms::UpdateTileTransforms, win_system::EndOfGameSystem,
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
    LoggerConfig,
};
use log::LevelFilter;
use structopt::StructOpt;
// use steamworks::{Client, FriendFlags};

#[macro_use]
extern crate lazy_static;

mod components;
mod file_utils;
mod high_scores;
mod level;
mod level_editor;
mod states;
mod systems;
mod tag;
mod ui_input;

///The width of the grid of tiless
pub const WIDTH: u32 = 64;
///The height of the grid of tiles
pub const HEIGHT: u32 = 36;
///The width of the grid of tiles in px relative to the spritesheet
pub const ARENA_WIDTH: u32 = 8 * WIDTH;
///The height of the grid of tiles in px relative to the spritesheet
pub const ARENA_HEIGHT: u32 = 8 * HEIGHT; //each sprite is 8px wide, so arena will be 16 sprites by 9 sprites
///The colour when a txt is hovered over
pub const HOVER_COLOUR: [f32; 4] = [1.0, 0.5, 0.75, 1.0];

fn main() -> amethyst::Result<()> {
    let opts = Flags::from_args();

    amethyst::start_logger(if opts.console {
        LoggerConfig::default()
    } else {
        let mut logger = LoggerConfig::default();
        logger.level_filter = LevelFilter::Error;
        logger.log_gfx_backend_level = Some(LevelFilter::Error);
        logger.log_gfx_rendy_level = Some(LevelFilter::Error);
        logger
    });

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");

    let mut game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear(get_colours(34.0, 35.0, 35.0)), //In Hex: 222223
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(UpdateTileTransforms, "update_tile_transforms", &[])
        .with(ListSystem, "collider_list", &[])
        .with(
            MovePlayerSystem::default(),
            "move_player",
            &["collider_list", "update_tile_transforms"],
        )
        .with(EndOfGameSystem, "end_of_game", &["collider_list"])
        .with(TextWobbleSystem, "txt_wobble", &[])
        .with(ScoreUpdaterSystem, "score_updater", &[])
        .with(GameModeTinterSystem, "tinter", &[]);

    if opts.fps {
        game_data = game_data.with(FpsCounterSystem, "fps", &[]).with(
            FpsPrinterSystem,
            "fps_printer",
            &["fps"],
        );
    }

    // let (client, single) = Client::init().unwrap();
    // println!("{:?}", client.friends().get_friends(FriendFlags::IMMEDIATE));

    let mut game = {
        if std::fs::read_dir(DATA_DIR).is_ok() {
            Application::new(resources, StartGameState::default(), game_data)?
        } else {
            std::fs::create_dir(DATA_DIR)
                .unwrap_or_else(|err| log::warn!("Unable to create data dir: {}", err));
            Application::new(resources, HelpState, game_data)?
        }
    };
    game.run();

    Ok(())
}

pub fn get_colours(r: f32, g: f32, b: f32) -> [f32; 4] {
    let (r, g, b, a) = Srgba::new(r / 255.0, g / 255.0, b / 255.0, 1.0)
        .into_linear()
        .into_components();
    [r, g, b, a]
}

///Flags for Lonely Tribes
#[derive(StructOpt, Debug)]
pub struct Flags {
    ///Enable an FPS counter in the console
    #[structopt(short, long)]
    pub fps: bool,

    ///Enable the console
    #[structopt(short, long)]
    pub console: bool,

    ///Enable debug options (disables high scores)
    ///Similar to Valve svcheats
    #[structopt(short, long)]
    pub debug: bool,

    ///Starting level, requires debug mode
    #[structopt(short, long)]
    pub level: Option<usize>,

    ///Option to enable legacy movement
    #[structopt(short, long)]
    pub timed_movement: Option<f32>,

    ///Option to use the debug level, requires debug mode
    #[cfg(debug_assertions)]
    #[structopt(long)]
    pub debug_level: bool,

    #[cfg(not(debug_assertions))]
    #[structopt(skip = false)]
    pub debug_level: bool,
}

//todos

//TODO: tests
//TODO: rayon

//TODO: With Text, make sure to account for Screen Scaling

//TODO: Story

//TODO: Music/SFX

//TODO: Steam Page/Steamworks
