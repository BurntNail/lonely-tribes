use crate::{HEIGHT, WIDTH};
use std::fs::read_to_string;
use image::{GenericImageView, Rgba, DynamicImage};
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
    LeftWallDown,
    RightWallDown,
    Player,
    Orc,
    EmptyHeart,
    FullHeart,
    HalfHeart,
    Axe,
    Bow,
    Arrow,
    Door,
    Blank,
    Shrubbery,
    DarkShrubbery,
    Tree,
    WarpedTree
}

lazy_static!{
    static ref SPRITESHEET_SWATCH_HASHMAP: HashMap<Rgba<u8>, SpriteRequest> = {
        use SpriteRequest::*;
        let mut map = HashMap::new();
        
        let c = |r: u8, g: u8, b: u8| { return Rgba::from([r, g, b, 255]); };
        let mut  s = |r: u8, g: u8, b: u8, s: SpriteRequest| map.insert(c(r, g, b), s);

        //In Asperite there is a default palette of colours. These are in the same order, reading like English - Left to Right, Top to Bottom
        s(0, 0, 0, BackWall); //1
        s(34, 32, 52, BackWallLeftCorner);
        s(69, 40, 60, BackWallRightCorner);
        s(102, 57, 49, LeftWall);
        s(143, 86, 59, RightWall); //2
        s(223, 113, 38, FrontWallLeftCorner);
        s(217, 160, 102, FrontWallRightCorner);
        s(238, 195, 154, FrontWall);
        s(251, 242, 54, LeftWallDown);//3
        s(153, 229, 80, RightWallDown);
        s(106, 190, 48, Door);
        s(55, 148, 110, Axe);
        s(75, 105, 47, Shrubbery);//4
        s(82, 75, 36, DarkShrubbery);
        s(50, 60, 57, Tree);
        s(63, 63, 116, WarpedTree);
        s(48, 96, 130, Player); //5

        map
    };
}

impl SpriteRequest {


    ///Function to get the index on the spritesheet for a SpriteRequest
    pub fn get_spritesheet_index(&self) -> usize { //REMEMBER - AMETHYST GOES BY ROWS
        use SpriteRequest::*;
        match self {
            BackWall => 1,
            BackWallLeftCorner => 3,
            BackWallRightCorner => 0,
            LeftWall => 14,
            RightWall => 17,
            LeftWallDown => 45,
            RightWallDown => 44,
            FrontWall => 58,
            Door => 30,
            FrontWallLeftCorner => 28,
            FrontWallRightCorner => 31,
            Player => 4,
            Orc => 11,
            EmptyHeart => 89,
            FullHeart => 90,
            HalfHeart => 91,
            Axe => 63,
            Bow => 65,
            Arrow => 67,
            Shrubbery => 61,
            DarkShrubbery => 60,
            Tree => 74,
            WarpedTree => 77,
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
        let mut data = vec![vec![SpriteRequest::Blank; HEIGHT as usize]; WIDTH as usize];
        image::open(path).unwrap_or_else(|err| {
            log::error!("Image Error for Room {}: {}", path, err);
            return DynamicImage::new_bgr8(16, 9);
        })

            .pixels().for_each(|(x, y, px)| {
            let res = *SpriteRequest::from_colour_swatch(&px);
            if res != SpriteRequest::Blank {
                data[x as usize][y as usize] = res;
                log::info!("Loading {:?} for ({}, {})", res, x, y);
            }
        } );

        Self {data}
    }
}