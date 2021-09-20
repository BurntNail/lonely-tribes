#![windows_subsystem = "windows"] //removes console window

mod states;

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
use lonely_tribes_lib::{
    audio::Muzac,
    paths::{get_directory, is_end_user_build},
    CONFIG,
};
use lonely_tribes_systems::{
    colliders_list_system::ListSystem,
    fog_of_war::{FogOfWarSystem, LightListSystem},
    fps_counter::FpsPrinterSystem,
    move_player::MovePlayerSystem,
    player_overlap_checker::PlayerOverlapChecker,
    tint_animator::TintAnimatorSystem,
    txt_wobble_system::TextWobbleSystem,
    update_score::ScoreUpdaterSystem,
    update_tile_transforms::UpdateTileTransforms,
};
use states::{help_state::HelpState, welcome_state::StartGameState};

fn main() -> amethyst::Result<()> {
    let opts = *CONFIG;
    log::info!("Using {:?}", opts);

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

    let resources = if is_end_user_build() {
        app_root.join("assets")
    } else {
        app_root.join("../assets")
    };

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

    let mut game = {
        if std::fs::read_dir(get_directory(false)).is_ok() {
            Application::new(resources, StartGameState::default(), game_data)?
        } else {
            let p = get_directory(false);
            std::fs::create_dir(p.clone()).unwrap_or_else(|err| {
                log::warn!(
                    "Unable to create data dir: {}, p: {}",
                    err,
                    p.to_str().unwrap_or_default()
                )
            });
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

//todos

//TODO: Move fog of war into it's own crate

//TODO: Music/SFX

//TODO: Steam Page/Steamworks
