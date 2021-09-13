use crate::{components::tile_transform::TileTransform, Either};

///Enumeration for the current state of the game
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStateEnum {
    ///The game is over
    End {
        ///Which position the game was lost by - if None, then we won
        lost_position: Option<TileTransform>,
    },
    ///The game is still being played
    ToBeDecided,
}

impl Default for GameStateEnum {
    fn default() -> Self {
        Self::ToBeDecided
    }
}

///The mode for gameplay - not the game state or the win state, but the mode of gameplay
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GamePlayingMode {
    ///One move - all moves
    Boring,
    ///Collisions are ignored
    Nudger,
    ///All movements go in a random direction, but are free
    TradeOff,
    ///All movements go to a random place
    Crazy,
    ///Everything is a collider
    AllTheColliders,
}
impl Default for GamePlayingMode {
    fn default() -> Self {
        Self::Boring
    }
}
impl GamePlayingMode {
    pub fn get_no_moves(&self) -> i32 {
        match self {
            Self::Boring => 0,
            Self::Nudger => 2,
            Self::TradeOff => 1,
            Self::Crazy => 3,
            Self::AllTheColliders => 10,
        }
    }
    pub fn adds_to_score(&self) -> bool {
        !(self == &Self::TradeOff)
    }
}

///The struct that holds the current GamePlayingMode, as well as the number of moves left
pub struct GameModeManager {
    pub total_moves: i32,
    pub moves_left: i32,
    pub current_mode: GamePlayingMode,
}
impl Default for GameModeManager {
    fn default() -> Self {
        Self {
            total_moves: 10,
            moves_left: 10,
            current_mode: GamePlayingMode::default(),
        }
    }
}
impl GameModeManager {
    pub fn new(moves: i32) -> Self {
        Self {
            total_moves: moves,
            moves_left: moves,
            current_mode: GamePlayingMode::Boring,
        }
    }

    pub fn do_move(&mut self) {
        self.moves_left -= self.current_mode.get_no_moves();
        self.get_and_update_mode();
    }

    pub fn set_mode(&mut self, nu_mode: GamePlayingMode) -> bool {
        if self.moves_left > 0 {
            self.current_mode = nu_mode;
            log::info!("Mode is now {:?}", nu_mode);
            true
        } else {
            false
        }
    }

    pub fn get_and_update_mode(&mut self) -> GamePlayingMode {
        if self.moves_left <= 0 {
            self.current_mode = GamePlayingMode::Boring;
        }
        self.current_mode
    }
}

//for the thing in the world, have a struct with a tuple of 'hacker' moves left, and total 'hacker' moves, and the current mode
//if we aren't in normal, subtract a certain amount depending on the new mode (maybe more moves for OP modes)
//then, when in hacker mode, make a system to set the opacity of the tint components on the hacker effects
//when the total reaches nought, there are no hacker moves left
//no numbers, just the alpha to show
//set amount per level

///Struct to hold the Win State Enum
#[derive(Clone, Debug)]
pub struct GameState {
    ///Current Game Win State
    pub ws: GameStateEnum,
    ///The level for which *ws* refers to
    pub level_from: Either<usize, u32>,
    ///Amount of time the level has taken
    pub level_no_of_moves: i32,
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            ws: GameStateEnum::default(), //Move this into it's own thing that the world reads
            level_from: Either::One(0),
            level_no_of_moves: 0,
        }
    }
}
impl GameState {
    ///Constructor for GameState with custom arguments
    ///
    ///  - **won_opt** is an option for whether or not the game has been won. If it is None, the game is still being played, or is being started, and if it is Some, then whether the game has been won is the bool
    ///  - **level_from** is for the level that won_opt refers to
    ///  - **level_timer_len** refers to how long the level took
    pub fn new(
        lost_tile: Option<Option<TileTransform>>,
        level_from: Either<usize, u32>,
        level_timer_len: i32,
    ) -> Self {
        match lost_tile {
            None => Self {
                ws: GameStateEnum::default(),
                level_from,
                level_no_of_moves: level_timer_len,
            },
            Some(tile) => Self {
                ws: GameStateEnum::End {
                    lost_position: tile,
                },
                level_from,
                level_no_of_moves: level_timer_len,
            },
        }
    }
}
