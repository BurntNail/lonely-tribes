use crate::procedural_generator::ProceduralGenerator;
use derive_try_from_primitive::TryFromPrimitive;
use image::{GenericImageView, Rgba};
use lonely_tribes_lib::{either::Either, paths::get_directory, HEIGHT, WIDTH};
use lonely_tribes_tags::{
    tag::{
        Tag,
        Tag::{Collision, Other},
    },
    trigger_type::TriggerType,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::read_to_string,
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum SpriteRequest {
    BackWall = 19,
    BackWallLeftCorner = 18,
    BackWallRightCorner = 20,
    LeftWall = 66,
    RightWall = 68,
    // LeftWallDown = 1,
    RightWallDown = 162,
    // LeftWallUp = 1,
    // RightWallUp = 1,
    FrontWall = 115,
    Door = 488,
    FrontWallLeftCorner = 114,
    FrontWallRightCorner = 116,
    Player0 = 409,
    Player1 = 361,
    Player2 = 366,
    Player3 = 367,
    Shrubbery = 96,
    DarkShrubbery = 4,
    Tree = 50,
    WarpedTree = 102,
    Blank = -1,
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
            SpriteRequest::Player0 => Self::Player(0),
            SpriteRequest::Player1 => Self::Player(1),
            SpriteRequest::Player2 => Self::Player(2),
            SpriteRequest::Player3 => Self::Player(3),
            SpriteRequest::Door => Self::Trigger(TriggerType::Door),
            Blank | Shrubbery | DarkShrubbery => Other,
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
        // s(251, 242, 54, LeftWallDown);//3
        // s(153, 229, 80, RightWallDown);
        s(106, 190, 48, Door);
        s(75, 105, 47, Shrubbery);//4
        s(82, 75, 36, DarkShrubbery);
        s(50, 60, 57, Tree);
        s(63, 63, 116, WarpedTree);
        s(48, 96, 130, Player0); //5
        s(91, 110, 225, Player1);
        s(99, 155, 255, Player2);
        s(95, 205, 228, Player3);


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
            // LeftWallDown,
            RightWallDown,
            // LeftWallUp,
            // RightWallUp,
            Player0,
            Player1,
            Player2,
            Player3,
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

            map.insert(sp as i32 as usize, sp);
        }

        map
    };
}

impl SpriteRequest {
    pub fn from_colour_swatch(col: &Rgba<u8>) -> &Self {
        SPRITESHEET_SWATCH_HASHMAP
            .get(col)
            .unwrap_or(&SpriteRequest::Blank)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    pub data: Vec<Vec<Either<SpriteRequest, i32>>>,
}
impl Default for Room {
    fn default() -> Self {
        Self {
            data: vec![vec![Either::One(SpriteRequest::Blank); WIDTH as usize]; HEIGHT as usize],
        }
    }
}

impl Deref for Room {
    type Target = Vec<Vec<Either<SpriteRequest, i32>>>;

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
        let mut data =
            vec![vec![Either::One(SpriteRequest::Blank); HEIGHT as usize]; WIDTH as usize];
        let path = get_directory(false).join("../maps").join(path);
        let path = path.to_str().unwrap_or_default();

        let img = image::open(path);
        match img {
            Ok(img) => img.pixels().for_each(|(x, y, px)| {
                let res = *SpriteRequest::from_colour_swatch(&px);
                if res != SpriteRequest::Blank {
                    data[x as usize][y as usize] = Either::One(res);
                }
            }),
            Err(_) => {
                let contents = read_to_string(path).unwrap_or_else(|err| {
                    log::error!("Image Error for Room {}: {}", path, err);
                    String::default()
                });

                for (y, line) in contents.lines().into_iter().enumerate() {
                    for (x, thing) in line.split(',').into_iter().enumerate() {
                        let i = thing.parse().unwrap_or(-1);
                        if i == -1 {
                            continue;
                        }

                        let spr = SpriteRequest::try_from(i);
                        data[x][y] = match spr {
                            Ok(spr) => Either::One(spr),
                            Err(_) => Either::Two(i),
                        };
                    }
                }
            }
        };

        Self { data }
    }

    pub fn proc_gen(seed: u32) -> Self {
        let mappings = ProceduralGenerator::new(seed).get();

        let mut data =
            vec![vec![Either::One(SpriteRequest::Blank); HEIGHT as usize]; WIDTH as usize];

        mappings.into_iter().for_each(|(x, y, spr)| {
            data[x][y] = Either::One(spr);
        });

        Self { data }
    }
}
