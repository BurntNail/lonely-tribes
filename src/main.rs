use amethyst::renderer::palette::Srgba;
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::UiBundle,
    utils::application_root_dir,
};
use crate::systems::UpdateTileTransforms;

mod level;
mod state;
mod systems;
mod components;

pub const WIDTH: u32 = 16;
pub const HEIGHT: u32 = 9;
pub const ARENA_WIDTH: u32 = 8 * WIDTH;
pub const ARENA_HEIGHT: u32 = 8 * HEIGHT; //each sprite is 8px wide, so arena will be 32 sprites by 18 sprites

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");
    let key_bindings_path = app_root.join("config/bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear(get_colours(34.0, 35.0, 35.0)),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(UpdateTileTransforms, "update_tile_transforms", &[]);

    let resources_path_str = format!("{:?}", resources.clone());
    let mut game = Application::new(resources, state::MyState::new(resources_path_str), game_data)?;
    game.run();

    Ok(())
}

fn get_colours(r_a: f32, g_a: f32, b_a: f32) -> [f32; 4] {
    let (r, g, b, a) = Srgba::new(r_a / 255., g_a / 255., b_a / 255., 1.0)
        .into_linear()
        .into_components();
    [r, g, b, a]
}
