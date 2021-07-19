use crate::{HEIGHT, WIDTH};
use std::fs::read_to_string;
use image::{GenericImageView, Rgba};
use std::collections::HashMap;

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
    Door,
    Blank,
}

lazy_static!{
    static ref SPRITESHEET_SWATCH_HASHMAP: HashMap<Rgba<u8>, SpriteRequest> = {
        let col = |r: u8, g: u8, b: u8| { return Rgba::from([r, g, b, 255]); };

        let mut map = HashMap::new();
        map.insert(col(20, 19, 21), SpriteRequest::BackWall);
        map
    };
}

impl SpriteRequest {
    pub fn get_spritesheet_index(&self) -> usize {
        use SpriteRequest::*;
        match self {
            BackWall => 1,
            BackWallLeftCorner => 0,
            BackWallRightCorner => 3,
            LeftWall => 14,
            RightWall => 17,
            FrontWall => 61,
            Door => 30,
            FrontWallLeftCorner => 29,
            FrontWallRightCorner => 32,
            Player => 4,
            Orc => 11,
            EmptyHeart => 89,
            FullHeart => 90,
            HalfHeart => 91,
            Sword => 63,
            Bow => 65,
            Arrow => 67,
            Blank => 9999,
        }
    }

    pub fn from_colour_swatch(col: &Rgba<u8>) -> &Self {
        SPRITESHEET_SWATCH_HASHMAP.get(col).unwrap_or(&SpriteRequest::Blank)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    pub data: Vec<Vec<SpriteRequest>>,
    // neighbours: HashMap<((usize, usize)), Room>
}

impl Room {
    pub fn new (path: &str) -> Self {
        log::info!("Loading Room: {}", path);

        let img = image::open(path).expect("Map not found");

        let mut data = vec![vec![SpriteRequest::Blank; HEIGHT as usize]; WIDTH as usize];

        img.pixels().for_each(|(x, y, px)| data[x as usize][y as usize] = *SpriteRequest::from_colour_swatch(&px) );

        Self {data}
    }
}