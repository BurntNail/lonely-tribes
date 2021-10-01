use crate::procedural_generator::ProceduralGenerator;
use image::{DynamicImage, GenericImageView, Rgba};
use lonely_tribes_lib::{paths::get_directory, HEIGHT, WIDTH};
use lonely_tribes_tags::{
    tag::{
        Tag,
        Tag::{Collision, Floor},
    },
    trigger_type::TriggerType,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    LeftWallUp,
    RightWallUp,
    TUpDownLeft,
    TUpDownRight,
    Player(usize),
    Door,
    Blank,
    Shrubbery,
    DarkShrubbery,
    Tree,
    WarpedTree,
}
impl Default for SpriteRequest {
    fn default() -> Self {
        Self::Blank
    }
}
impl Default for &SpriteRequest {
    fn default() -> Self {
        &SpriteRequest::Blank
    }
}

pub trait FromSpr {
    type Output;
    fn from_spr(spr: SpriteRequest) -> Self::Output;
}
impl FromSpr for Tag {
    type Output = Self;
    fn from_spr(spr: SpriteRequest) -> Self {
        use SpriteRequest::*;
        match spr {
            SpriteRequest::Player(id) => Self::Player(id),
            SpriteRequest::Door => Self::Trigger(TriggerType::Door),
            Blank | Shrubbery | DarkShrubbery => Floor,
            _ => Collision,
        }
    }
}

lazy_static! {
    static ref SPRITESHEET_SWATCH_HASHMAP: HashMap<Rgba<u8>, SpriteRequest> = {
        use SpriteRequest::*;
        let mut map = HashMap::new();

        let c = |r: u8, g: u8, b: u8| { Rgba::from([r, g, b, 255]) };
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
        s(75, 105, 47, Shrubbery);//4
        s(82, 75, 36, DarkShrubbery);
        s(50, 60, 57, Tree);
        s(63, 63, 116, WarpedTree);
        s(48, 96, 130, Player(0)); //5
        s(91, 110, 225, Player(1));
        s(99, 155, 255, Player(2));
        s(95, 205, 228, Player(3));
        s(203, 219, 252, TUpDownLeft); //6
        s(255, 255, 255, TUpDownRight);


        map
    };

    pub static ref REVERSED_SPRITESHEET_SWATCH_HASHMAP: HashMap<SpriteRequest, Rgba<u8>> = {
        let mut map = HashMap::new();

        SPRITESHEET_SWATCH_HASHMAP.clone().into_iter().for_each(|(v, k)| {
            map.insert(k, v);
        });

        map
    };

    pub static ref LIST_OF_ALL_SPRITEREQUESTS: Vec<SpriteRequest> = {
        use SpriteRequest::*;
        vec![BackWall,
            BackWallLeftCorner,
            BackWallRightCorner,
            LeftWall,
            RightWall,
            FrontWall,
            FrontWallLeftCorner,
            FrontWallRightCorner,
            LeftWallDown,
            RightWallDown,
            LeftWallUp,
            RightWallUp,
            TUpDownLeft,
            TUpDownRight,
            Player(0),
            Player(1),
            Player(2),
            Player(3),
            Door,
            Blank,
            Shrubbery,
            DarkShrubbery,
            Tree,
            WarpedTree,
        ]
    };

    pub static ref INDEX_TO_SPRITEREQUEST: HashMap<usize, SpriteRequest> = {
        let mut map = HashMap::new();

        for sp in LIST_OF_ALL_SPRITEREQUESTS.clone() {
            map.insert(sp.get_spritesheet_index(), sp);
        }

        map
    };
}

impl SpriteRequest {
    ///Function to get the index on the spritesheet for a SpriteRequest
    pub fn get_spritesheet_index(&self) -> usize {
        //REMEMBER - AMETHYST GOES BY ROWS
        use SpriteRequest::*;
        match self {
            BackWall => 19,
            BackWallLeftCorner => 18,
            BackWallRightCorner => 20,
            LeftWall => 66,
            RightWall => 68,
            LeftWallDown => 1, //fix
            RightWallDown => 162,
            LeftWallUp => 1,  //fix
            RightWallUp => 1, //fix
            FrontWall => 115,
            Door => 488,
            FrontWallLeftCorner => 114,  //fix
            FrontWallRightCorner => 116, //fix
            Player(index) => match index {
                0 => 409,
                1 => 361,
                2 => 366,
                3 => 367,
                _ => 0,
            },
            TUpDownLeft => 71,  //fix
            TUpDownRight => 70, //fix
            Shrubbery => 96,
            DarkShrubbery => 4,
            Tree => 51,
            WarpedTree => 102,
            Blank => 0,
        }
    }

    pub fn from_colour_swatch(col: &Rgba<u8>) -> &Self {
        SPRITESHEET_SWATCH_HASHMAP
            .get(col)
            .unwrap_or(&SpriteRequest::Blank)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    pub data: Vec<Vec<SpriteRequest>>,
}
impl Default for Room {
    fn default() -> Self {
        Self {
            data: vec![vec![SpriteRequest::Blank; WIDTH as usize]; HEIGHT as usize],
        }
    }
}

impl Deref for Room {
    type Target = Vec<Vec<SpriteRequest>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Room {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Room {
    pub fn new(path: String) -> Self {
        let mut data = vec![vec![SpriteRequest::Blank; HEIGHT as usize]; WIDTH as usize];
        let path = get_directory(false).join("../maps").join(path);
        let path = path.to_str().unwrap_or_default();

        image::open(path)
            .unwrap_or_else(|err| {
                log::error!("Image Error for Room {}: {}", path, err);
                DynamicImage::new_bgr8(WIDTH as u32, HEIGHT as u32)
            })
            .pixels()
            .for_each(|(x, y, px)| {
                let res = *SpriteRequest::from_colour_swatch(&px);
                if res != SpriteRequest::Blank {
                    data[x as usize][y as usize] = res;
                }
            });

        Self { data }
    }

    pub fn proc_gen(seed: u32) -> Self {
        let mappings = ProceduralGenerator::new(seed).get();

        let mut data = vec![vec![SpriteRequest::Blank; HEIGHT as usize]; WIDTH as usize];

        mappings.into_iter().for_each(|(x, y, spr)| {
            data[x][y] = spr;
        });

        Self { data }
    }
}
