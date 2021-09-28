use crate::sprite_stuff::Room;
use serde::Deserialize;
use ron::from_str;
use std::fs::read_to_string;
use lonely_tribes_lib::paths::get_directory;
use lonely_tribes_lib::either::Either;

#[derive(Deserialize, Debug)]
pub struct ReadInLevel {
	path: Option<String>,
	seed: Option<u32>,
	specials: usize
}

#[derive(Debug)]
pub struct Level {
	pub room: Room,
	pub specials: usize,
}

impl Level {
	pub fn get_seed_index_from_path (path: &str) -> Either<usize, u32> {
		let pathbuf = get_directory(false).join("../maps").join(path);
		let contents = read_to_string(pathbuf).unwrap_or_default();
		let ril = from_str::<ReadInLevel>(&contents);

		match ril {
			Ok(good) => {
				if let Some(i) = good.path {
					let res = i.replace("lvl-", "").replace(".ron", "").parse().unwrap_or_default();
					Either::One(res)
				} else if let Some(s) = good.seed {
					Either::Two(s)
				} else {
					Either::Two(0)
				}
			}
			Err(_) => Either::Two(0)
		}
	}

	pub fn new (path: &str) -> Self {
		let pathbuf = get_directory(false).join("../maps").join(path);
		let contents = read_to_string(pathbuf).unwrap_or_default();
		let ril = from_str::<ReadInLevel>(&contents);

		match ril {
			Ok(ok) => {
				let room = if let Some(p) = ok.path {
					Room::new(p)
				} else if let Some(s) = ok.seed {
					Room::proc_gen(s)
				} else {
					Room::default()
				};

				Self {
					room,
					specials: ok.specials
				}
			},
			Err(err) => {
				log::warn!("Error reading in room: {}", err);
				Self {
					room: Room::default(),
					specials: 50
				}
			}
		}
	}
}