use crate::{components::power_up::PowerUp, HEIGHT, WIDTH};
use image::{DynamicImage, GenericImageView, Rgba};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[allow(dead_code)]
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
    Orc,
    FullHeart,
    Axe,
    Door,
    Blank,
    Shrubbery,
    DarkShrubbery,
    Tree,
    WarpedTree,
    PowerUpSprite(PowerUp),
}
impl Default for SpriteRequest {
    fn default() -> Self {
        Self::Blank
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
        s(55, 148, 110, Orc);
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


        s(105, 106, 106, PowerUpSprite(PowerUp::ScoreChanger)); //7
        s(89, 86, 82, PowerUpSprite(PowerUp::Reaper));
        s(118, 66, 138, PowerUpSprite(PowerUp::Portal));

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
        use PowerUp::*;
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
            Orc,
            FullHeart,
            Axe,
            Door,
            Blank,
            Shrubbery,
            DarkShrubbery,
            Tree,
            WarpedTree,
            PowerUpSprite(Portal),
            PowerUpSprite(Reaper),
            PowerUpSprite(ScoreChanger),
        ]
    };
}

impl SpriteRequest {
    ///Function to get the index on the spritesheet for a SpriteRequest
    pub fn get_spritesheet_index(&self) -> usize {
        //REMEMBER - AMETHYST GOES BY ROWS
        use SpriteRequest::*;
        match self {
            BackWall => 1,
            BackWallLeftCorner => 3,
            BackWallRightCorner => 0,
            LeftWall => 14,
            RightWall => 17,
            LeftWallDown => 45,
            RightWallDown => 44,
            LeftWallUp => 43,
            RightWallUp => 42,
            FrontWall => 58,
            Door => 30,
            FrontWallLeftCorner => 28,
            FrontWallRightCorner => 31,
            Player(index) => match index {
                0 => 4,
                1 => 5,
                2 => 6,
                3 => 7,
                _ => 9999,
            },
            PowerUpSprite(t) => match t {
                PowerUp::Reaper => 23,
                PowerUp::ScoreChanger => 13,
                PowerUp::Portal => 113,
            },
            TUpDownLeft => 71,
            TUpDownRight => 70,
            Orc => 11,
            FullHeart => 90,
            Axe => 63,
            Shrubbery => 61,
            DarkShrubbery => 60,
            Tree => 74,
            WarpedTree => 77,
            Blank => 9999,
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
    pub fn new(path: &str) -> Self {
        let mut data = vec![vec![SpriteRequest::Blank; HEIGHT as usize]; WIDTH as usize];
        let path = format!("assets/maps/{}", path);

        image::open(path.clone())
            .unwrap_or_else(|err| {
                log::error!("Image Error for Room {}: {}", path, err);
                DynamicImage::new_bgr8(WIDTH, HEIGHT)
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
}
