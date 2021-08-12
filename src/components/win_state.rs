///Enumeration for the current state of the game
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStateEnum {
    ///The game is over
    End {
        ///Whether the game was won or not
        won: bool,
    },
    ///The game is still being played
    ToBeDecided(GamePlayingMode),
}

///The mode for gameplay - not the game state or the win state, but the mode of gameplay
pub enum GamePlayingMode {
    ///One move - all moves
    Normal,
    ///Collisions are ignored (inc OOB)
    Nudger
}

impl Default for GameStateEnum {
    fn default() -> Self {
        Self::ToBeDecided(GamePlayingMode::Normal)
    }
}
///Struct to hold the Win State Enum
#[derive(Clone, Debug)]
pub struct GameState {
    ///Current Game Win State
    pub ws: GameStateEnum,
    ///The level for which *ws* refers to
    pub level_from: usize,
    ///Amount of time the level has taken
    pub level_no_of_moves: i32,

}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            ws: GameStateEnum::default(),
            level_from: 0,
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
    pub fn new(won_opt: Option<bool>, level_from: usize, level_timer_len: i32) -> Self {
        match won_opt {
            None => Self {
                ws: GameStateEnum::default(),
                level_from,
                level_no_of_moves: level_timer_len,
            },
            Some(won) => Self {
                ws: GameStateEnum::End { won },
                level_from,
                level_no_of_moves: level_timer_len,
            },
        }
    }
}
