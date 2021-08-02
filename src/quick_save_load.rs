use crate::components::{Player, TileTransform, PowerUp};
use ron::{to_string, from_str};
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::{
	fs::{create_dir, write},
};
use crate::high_scores::DATA_DIR;
use std::fs::{read_dir, read_to_string};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelState {
	pub players: Vec<(Player, TileTransform)>,
	pub powerups: Vec<(PowerUp, TileTransform)>,
	pub score: i32
}

#[derive(Copy, Clone, Debug)]
pub enum SaveType {
	QuickSave,
	ManualSave
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
	pub fn replace (&mut self, players: Vec<(Player, TileTransform)>, powerups: Vec<(PowerUp, TileTransform)>, score: i32)  {
		self.players = players;
		self.powerups = powerups;
		self.score = score;
	}

	pub fn save (&self, save_type: SaveType, level: usize) {
		let text = to_string(&self).unwrap_or_else(|err| {
			log::error!("Error serialising LevelState: {:?}", err);
			"".to_string()
		});
		let file_name = match save_type {
			SaveType::QuickSave => format!("{}/QuickSave-{:02}.ron", DATA_DIR, level),
			_ => format!("{}/ManualSave-{}-LEVEL{:02}.ron", DATA_DIR, Local::now().format("[%Y-%m-%d][%H:%M:%S]"), level),
		};

		log::info!("New Save - {}, which contains {}", file_name, text);

		write(file_name.clone(), text.clone()).unwrap_or_else(|_| {
			create_dir(DATA_DIR).unwrap_or_else(|err| log::error!("Unable to create data directory: {}", err));
			write(file_name, text.clone()).unwrap_or_else(|err| log::error!("Unable to {:?}: {}", save_type, err));
		})
	}

	pub fn load_most_recent (save_type: Option<SaveType>, level: usize) -> Option<Self> {
		let mut save_file_name = None;
		if let Ok(read) = read_dir(DATA_DIR) {
			let mut list = Vec::new();
			read.for_each(|el| {
				if let Ok(el) = el {
					let current_file = format!("{:?}", el.file_name());
					if current_file.contains(&format!("{:02}", level)) {
						if let Some(save_type) = save_type {
							if current_file.contains(&save_type.to_string()) {
								list.push(current_file);
							}
						} else {
							list.push(current_file);
						}

					}
				}
			});

			list.sort();
			let level = list.last();
			if let Some(level) = level {
				let initial_name = format!("{}/{}", DATA_DIR, level);
				save_file_name = Some(initial_name.replace("\"", ""));
			}
		}

		let mut res = None;
		if let Some(save_file_name) = save_file_name {
			match read_to_string(save_file_name) {
				Ok(save_file) => {
					match from_str(save_file.as_str()) {
						Ok(v) => res = Some(v),
						Err(e) => log::warn!("Couldn't serialise in level - {}", e),
					}
				},
				Err(e) => log::warn!("Couldn't read in save - {}", e)
			}
		}

		res
	}
}
impl Default for LevelState {
	fn default() -> Self {
		Self {
			players: Vec::new(),
			powerups: Vec::new(),
			score: 0
		}
	}
}
