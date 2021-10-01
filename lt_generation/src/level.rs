use crate::sprite_stuff::Room;
use lonely_tribes_lib::{either::Either, paths::get_directory};
use ron::from_str;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadInLevel {
    pub seed: Option<u32>,
    pub specials: usize,
    pub messages: Vec<(f32, String)>,
}

#[derive(Debug)]
pub struct Level {
    pub room: Room,
    pub specials: usize,
    pub messages: Vec<(f32, String)>,
}

pub const RT_PROCGEN_FILENAME: &str = "runtime-procgen";

impl Level {
    pub fn get_seed_index_from_path(path: &str) -> Either<usize, u32> {
        let pathbuf = get_directory(false).join("../maps").join(path);
        let contents = read_to_string(pathbuf).unwrap_or_default();
        let ril = from_str::<ReadInLevel>(&contents);

        match ril {
            Ok(good) => {
                if let Some(s) = good.seed {
                    Either::Two(s)
                } else {
                    let res: usize = path
                        .replace("lvl-", "")
                        .replace(".ron", "")
                        .parse()
                        .unwrap_or_default();

                    Either::One(res - 1)
                }
            }
            Err(_) => Either::Two(0),
        }
    }

    pub fn new(path: &str) -> (Self, Option<u32>) {
        if path.contains(RT_PROCGEN_FILENAME)
        //if we don't have a path, cos we are doing procgen now
        {
            let seed = rand::random();
            return (
                Self {
                    room: Room::proc_gen(seed),
                    specials: 50,
                    messages: Vec::new(),
                },
                Some(seed),
            );
        }

        let pathbuf = get_directory(false).join("../maps").join(path);
        let contents = read_to_string(&pathbuf).unwrap_or_default();
        let ril = from_str::<ReadInLevel>(&contents);

        let r = match ril {
            Ok(ok) => {
                let room = if let Some(s) = ok.seed {
                    Room::proc_gen(s)
                } else {
                    Room::new(path.replace(".ron", ".png"))
                };

                Self {
                    room,
                    specials: ok.specials,
                    messages: ok.messages,
                }
            }
            Err(err) => {
                log::warn!("Error reading in room: {} at path: {:?}", err, pathbuf);
                Self {
                    room: Room::default(),
                    specials: 0,
                    messages: Vec::new(),
                }
            }
        };
        (r, None)
    }
}
