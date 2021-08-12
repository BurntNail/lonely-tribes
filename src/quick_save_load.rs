use crate::{
    components::{player::Player, power_up::PowerUp, tile_transform::TileTransform},
    high_scores::DATA_DIR,
};
use chrono::Local;
use ron::{from_str, to_string};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    fs::{create_dir, read_dir, read_to_string, write},
    path::Path,
};

///Struct to store state of a level for quick-save/loading
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelState {
    pub players: Vec<(Player, TileTransform)>,
    pub powerups: Vec<(PowerUp, TileTransform)>,
    pub score: i32,
}

///The type of save
#[derive(Copy, Clone, Debug)]
pub enum SaveType {
    ///Quick saves - only one stored at a time - each new save overrides the old one
    QuickSave,
    ///Manual saves - many at a time
    #[allow(dead_code)]
    ManualSave,
}
impl Display for SaveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveType::QuickSave => write!(f, "QuickSave"),
            SaveType::ManualSave => write!(f, "ManualSave"),
        }
    }
}

impl LevelState {
    ///Replaces current data with new data
    pub fn replace(
        &mut self,
        players: Vec<(Player, TileTransform)>,
        powerups: Vec<(PowerUp, TileTransform)>,
        score: i32,
    ) {
        self.players = players;
        self.powerups = powerups;
        self.score = score;
    }

    ///Save the current level_state into a file
    pub fn save(&self, save_type: SaveType, level: usize) {
        let text = to_string(&self).unwrap_or_else(|err| {
            log::error!("Error serialising LevelState: {:?}", err);
            "".to_string()
        });
        let file_name = match save_type {
            SaveType::QuickSave => format!("{}/QuickSave-{:02}.ron", DATA_DIR, level),
            _ => format!(
                "{}/ManualSave-{}-LEVEL{:02}.ron",
                DATA_DIR,
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                level
            ),
        };

        write(&file_name, &text).unwrap_or_else(|_| {
            create_dir(DATA_DIR)
                .unwrap_or_else(|err| log::error!("Unable to create data directory: {}", err));
            write(&file_name, &text)
                .unwrap_or_else(|err| log::error!("Unable to {:?}: {}", save_type, err));
        })
    }

    ///Load the most recent save (of a given type, or any type if given is null) for a level. Returns None if there isn't a save or there's an error with it, and Some with the level state
    pub fn load_most_recent(save_type: Option<SaveType>, level: usize) -> Option<Self> {
        let mut save_file_name = None;
        let full_list = Self::list_file_names_in_dir(DATA_DIR);
        let mut list = Vec::new();
        if !full_list.is_empty() {
            full_list.into_iter().for_each(|current_file| {
                if current_file.contains(&format!("{:02}", level)) {
                    if let Some(save_type) = save_type {
                        if current_file.contains(&save_type.to_string()) {
                            list.push(current_file);
                        }
                    } else {
                        list.push(current_file);
                    }
                }
            });

            let level = list.last();
            if let Some(level) = level {
                let initial_name = format!("{}/{}", DATA_DIR, level);
                save_file_name = Some(initial_name.replace("\"", ""));
            }
        }

        let mut res = None;
        if let Some(save_file_name) = save_file_name {
            match read_to_string(save_file_name) {
                Ok(save_file) => match from_str(save_file.as_str()) {
                    Ok(v) => res = Some(v),
                    Err(e) => log::warn!("Couldn't serialise in level - {}", e),
                },
                Err(e) => log::warn!("Couldn't read in save - {}", e),
            }
        }

        res
    }

    ///Gets file names inside a directory
    pub fn list_file_names_in_dir<P: AsRef<Path>>(path: P) -> Vec<String> {
        let mut list = Vec::new();
        if let Ok(read) = read_dir(path) {
            read.for_each(|el| {
                if let Ok(el) = el {
                    let current_file = format!("{:?}", el.file_name());
                    list.push(current_file);
                }
            });

            list.sort();
            list.reverse();
        }

        list
    }
}

impl Default for LevelState {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            powerups: Vec::new(),
            score: 0,
        }
    }
}
