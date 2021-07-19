use crate::{HEIGHT, WIDTH};
use amethyst::assets::{Asset, Handle, ProcessableAsset, ProcessingState};
use amethyst::core::ecs::DenseVecStorage;
use amethyst::Error;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::{BufReader, BufRead};
use ron::de::from_str;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SpriteRequest {
    BackWall,
    BackWallLeftCorner,
    BackWallRightCorner,
    LeftWall,
    RightWall,
    FrontWall,
    FrontWallLeftCorner,
    FrontWallRightCorner,
    Player,
    Orc,
    EmptyHeart,
    FullHeart,
    HalfHeart,
    Sword,
    Bow,
    Arrow,
    Blank,
}

// pub enum Dir {
//     Up,
//     Down,
//     Left,
//     Right
// }

impl SpriteRequest {
    pub fn get_index(&self) -> usize {
        use SpriteRequest::*;
        match self {
            BackWall => 0,
            BackWallLeftCorner => 1,
            BackWallRightCorner => 2,
            LeftWall => 3,
            RightWall => 4,
            FrontWall => 5,
            FrontWallLeftCorner => 6,
            FrontWallRightCorner => 7,
            Player => 8,
            Orc => 9,
            EmptyHeart => 10,
            FullHeart => 11,
            HalfHeart => 12,
            Sword => 13,
            Bow => 14,
            Arrow => 15,
            Blank => 16,
        }
    }

    pub fn at_index(i: &usize) -> Self {
        use SpriteRequest::*;
        match i {
            0 => BackWall,
            1 => BackWallLeftCorner,
            2 => BackWallRightCorner,
            3 => LeftWall,
            4 => RightWall,
            5 => FrontWall,
            6 => FrontWallLeftCorner,
            7 => FrontWallRightCorner,
            8 => Player,
            9 => Orc,
            10 => EmptyHeart,
            11 => FullHeart,
            12 => HalfHeart,
            13 => Sword,
            14 => Bow,
            15 => Arrow,
            _ => Blank,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ReadInRoom {
    map: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    pub data: Vec<Vec<SpriteRequest>>,
    // neighbours: HashMap<((usize, usize)), Room>
}

impl Room {
    pub fn new (path: &str) -> Self {
        log::info!("Loading Room: {}", path);
        let file_contents = read_to_string(path).unwrap_or("".to_string());
        let raw_room = from_str(file_contents.as_str()).unwrap_or(ReadInRoom::default());

        let mut res: Vec<Vec<SpriteRequest>> = Vec::new();
        let mut i = 0;
        for _x in 0..WIDTH {
            let mut col = Vec::new();
            for _y in 0..HEIGHT {
                let spr_index = raw_room.map.get(i).unwrap_or_else(|| {
                    // log::warn!("Room Data for index_{} not found", i);
                    &16
                });
                col.push(SpriteRequest::at_index(spr_index));
                i += 1;
            }
            res.push(col);
        }

        Self { data: res }

    }
}

//region using the room as an asset - failed
// impl Asset for Room {
//     const NAME: &'static str = "rougelike-rpg::Room";
//     type Data = ReadInRoom;
//     type HandleStorage = DenseVecStorage<Handle<Room>>;
// }
//
// impl ProcessableAsset for Room {
//     fn process(raw: Self::Data) -> Result<ProcessingState<Self>, Error> {
//         let mut res: Vec<Vec<SpriteRequest>> = Vec::new();
//         let mut i = 0;
//         for _x in 0..WIDTH {
//             let mut row = Vec::new();
//             for _y in 0..HEIGHT {
//                 let spr_index = raw.map.get(i).unwrap_or(&16);
//                 row.push(SpriteRequest::at_index(spr_index));
//                 i += 1;
//             }
//             res.push(row);
//         }
//
//         Ok(ProcessingState::Loaded(Self { data: res }))
//     }
// }
//endregion