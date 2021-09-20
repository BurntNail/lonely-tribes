pub mod audio;
pub mod config;
pub mod either;
pub mod high_scores;
pub mod paths;
pub mod states_util;
pub mod ui_input;

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

lazy_static::lazy_static! {
    pub static ref CONFIG: config::LTConfig = {
        config::LTConfig::new()
    };
}
