use crate::paths::get_directory;
use ron::from_str;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use structopt::StructOpt;

///Flags for Lonely Tribes
#[derive(StructOpt, Copy, Clone, Debug)]
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

    ///Disable Fog Of War
    ///Requires debug
    #[structopt(long)]
    pub fow_disabled: bool,

    ///Option to enable held movement
    #[structopt(short, long)]
    pub timed_movement: bool,
}
impl Flags {
    pub fn fow_enabled(&self) -> bool {
        !(self.debug && self.fow_disabled)
    }
}

pub const DEFAULT_SCREEN_RES: (u32, u32) = (1920, 1080);
pub const DEFAULT_DPI: f64 = 1.0;

#[derive(Copy, Clone, Debug)]
pub struct LTConfig {
    pub flags: Flags,
    pub conf: ParsedConfig,
}
#[derive(Serialize, Deserialize, Debug)]
struct ReadInConfig {
    pub screen_dimensions: Option<(u32, u32)>,
    pub dpi_factor: Option<f64>,
    pub maximised: bool,
    pub vol: f32,
}
#[derive(Copy, Clone, Debug)]
pub struct ParsedConfig {
    pub screen_dimensions: (u32, u32),
    pub dpi_factor: f64,
    pub maximised: bool,
    pub vol: f32,
}
impl Default for ParsedConfig {
    fn default() -> Self {
        ParsedConfig {
            screen_dimensions: DEFAULT_SCREEN_RES,
            dpi_factor: DEFAULT_DPI,
            maximised: true,
            vol: 1.0,
        }
    }
}
impl From<ParsedConfig> for ReadInConfig {
    fn from(c: ParsedConfig) -> Self {
        ReadInConfig {
            screen_dimensions: Some(c.screen_dimensions),
            dpi_factor: Some(c.dpi_factor),
            maximised: c.maximised,
            vol: c.vol,
        }
    }
}
impl ParsedConfig {
    pub fn new() -> Self {
        let path = get_directory(true).join("conf.ron");
        let contents = read_to_string(path.clone()).unwrap_or_default();
        match from_str(contents.as_str()) {
            Ok(w) => {
                log::info!("Parsing conf: {:?}", w);
                let w: ReadInConfig = w;
                let sd = w.screen_dimensions.unwrap_or_else(|| {
                    log::warn!("Unable to parse screen dims");
                    DEFAULT_SCREEN_RES
                });
                let dpi_factor = w.dpi_factor.unwrap_or(DEFAULT_DPI);
                Self {
                    screen_dimensions: sd,
                    maximised: w.maximised,
                    vol: w.vol,
                    dpi_factor,
                }
            }
            Err(e) => {
                log::warn!(
                    "Unable to parse conf: {}, contents: {}, path: {}",
                    e,
                    contents,
                    path.to_str().unwrap_or_default()
                );
                Self::default()
            }
        }
    }
}
impl LTConfig {
    pub(crate) fn new() -> Self {
        Self {
            flags: Flags::from_args(),
            conf: ParsedConfig::new(),
        }
    }
}
impl Default for LTConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub fn change_screen_res(new_x: u32, new_y: u32) {
    log::info!("Changing to {}, {}", new_x, new_y);
    let mut conf = ParsedConfig::new();
    conf.screen_dimensions.0 = new_x;
    conf.screen_dimensions.1 = new_y;

    if let Ok(str_version) = ron::to_string(&ReadInConfig::from(conf)) {
        std::fs::write(get_directory(true).join("conf.ron"), str_version)
            .unwrap_or_else(|err| log::warn!("Unable to write new stuff to config: {}", err));
    }
}

pub fn change_screen(new_x: u32, new_y: u32, dpi: f64) {
    log::info!("Changing to {}, {}", new_x, new_y);
    let mut conf = ParsedConfig::new();
    conf.screen_dimensions.0 = new_x;
    conf.screen_dimensions.1 = new_y;
    conf.dpi_factor = dpi;

    if let Ok(str_version) = ron::to_string(&ReadInConfig::from(conf)) {
        std::fs::write(get_directory(true).join("conf.ron"), str_version)
            .unwrap_or_else(|err| log::warn!("Unable to write new stuff to config: {}", err));
    }
}
