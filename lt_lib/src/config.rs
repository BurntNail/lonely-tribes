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

#[derive(Copy, Clone, Debug)]
pub struct LTConfig {
    pub flags: Flags,
    pub conf: ParsedConfig,
}
#[derive(Serialize, Deserialize)]
struct ReadInConfig {
    pub screen_dimensions: Option<(u32, u32)>,
    pub maximised: bool,
    pub vol: f32,
}
#[derive(Copy, Clone, Debug)]
pub struct ParsedConfig {
    pub screen_dimensions: (u32, u32),
    pub maximised: bool,
    pub vol: f32,
}
impl Default for ParsedConfig {
    fn default() -> Self {
        ParsedConfig {
            screen_dimensions: (1600, 900),
            maximised: false,
            vol: 1.0,
        }
    }
}
impl ParsedConfig {
    pub fn new() -> Self {
        let path = get_directory(true).join("conf.ron");
        let contents = read_to_string(path.clone()).unwrap_or_default();
        match from_str(contents.as_str()) {
            Ok(w) => {
                let w: ReadInConfig = w;
                let sd = w.screen_dimensions.unwrap_or((1920, 1080));
                Self {
                    screen_dimensions: sd,
                    maximised: w.maximised,
                    vol: w.vol,
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