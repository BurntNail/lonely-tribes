#![windows_subsystem = "windows"] //removes console window

use crate::{
    audio::Muzac,
    config::LTConfig,
    high_scores::DATA_DIR,
    states::{help_state::HelpState, welcome_state::StartGameState},
    systems::{
        colliders_list_system::ListSystem,
        fog_of_war::{FogOfWarSystem, LightListSystem},
        fps_counter::FpsPrinterSystem,
        move_player::MovePlayerSystem,
        tint_animator::TintAnimatorSystem,
        txt_wobble_system::TextWobbleSystem,
        update_score::ScoreUpdaterSystem,
        update_tile_transforms::UpdateTileTransforms,
        player_overlap_checker::PlayerOverlapChecker,
    },
};
use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
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
    window::DisplayConfig,
    LoggerConfig,
};
use log::LevelFilter;
// use steamworks::*;

#[macro_use]
extern crate lazy_static;

mod audio;
mod components;
mod config;
mod file_utils;
mod high_scores;
mod level;
mod level_editor;
mod procedural_generator;
mod states;
mod systems;
mod tag;
mod ui_input;

pub const TILE_WIDTH_HEIGHT: i32 = 8;
///The width of the grid of tiless
pub const WIDTH: i32 = 64;
///The height of the grid of tiles
pub const HEIGHT: i32 = 36;
///The width of the grid of tiles in px relative to the spritesheet
pub const ARENA_WIDTH: i32 = TILE_WIDTH_HEIGHT * WIDTH;
///The height of the grid of tiles in px relative to the spritesheet
pub const ARENA_HEIGHT: i32 = TILE_WIDTH_HEIGHT * HEIGHT; //each sprite is 8px wide, so arena will be 16 sprites by 9 sprites
///The colour when a txt is hovered over
pub const HOVER_COLOUR: [f32; 4] = [1.0, 0.5, 0.75, 1.0];

fn main() -> amethyst::Result<()> {
    let opts = LTConfig::new();

    amethyst::start_logger(if opts.flags.console {
        LoggerConfig::default()
    } else {
        LoggerConfig {
            level_filter: LevelFilter::Off,
            log_gfx_backend_level: Some(LevelFilter::Off),
            log_gfx_rendy_level: Some(LevelFilter::Off),
            ..Default::default()
        }
    });

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = DisplayConfig {
        title: "Lonely Tribes".to_string(),
        dimensions: Some(opts.conf.screen_dimensions),
        maximized: opts.conf.maximised,
        decorations: !opts.conf.maximised,
        icon: Some(resources.join("art/logo.png")),
        ..Default::default()
    };

    let mut game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
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
        .with(PlayerOverlapChecker, "player_overlap", &[])
        .with(TextWobbleSystem, "txt_wobble", &[])
        .with(ScoreUpdaterSystem, "score_updater", &[])
        .with(LightListSystem, "light_list", &[])
        .with(FogOfWarSystem::default(), "fog_of_war", &["light_list"])
        .with(TintAnimatorSystem, "tint_animtor", &[]);

    if opts.flags.fps {
        game_data = game_data.with(FpsCounterSystem, "fps", &[]).with(
            FpsPrinterSystem,
            "fps_printer",
            &["fps"],
        );
    }
    if opts.conf.vol > 0.0 {
        game_data = game_data
            .with_bundle(AudioBundle::default())?
            .with_system_desc(
                DjSystemDesc::new(|music: &mut Muzac| music.music.next()),
                "dj_system",
                &[],
            );
    }

    // {
    //     let (client, single) = Client::init().unwrap();
    //
    //     for _ in 0..50 {
    //         single.run_callbacks();
    //         ::std::thread::sleep(::std::time::Duration::from_millis(100));
    //     }
    // }

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

#[derive(Copy, Clone, Debug)]
pub enum Either<T1, T2> {
    One(T1),
    Two(T2),
}
impl<T1, T2> Either<T1, T2> {
    pub fn is_one(&self) -> bool {
        matches!(self, Self::One(_))
    }
    pub fn is_two(&self) -> bool {
        matches!(self, Self::Two(_))
    }
}

//todos

//TODO: Story
//TODO: Levels

//TODO: Music/SFX

//TODO: Steam Page/Steamworks
