///Enumeration for the current state of the game
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WinStateEnum {
    ///The game is over
    End { won: bool },
    ///The game is still being played
    TBD,
}
impl Default for WinStateEnum {
    fn default() -> Self {
        Self::TBD
    }
}
///Struct to hold the Win State Enum
#[derive(Clone, Debug)]
pub struct GameWinState {
    ///Current Win State
    pub ws: WinStateEnum,
    ///The level for which *ws* refers to
    pub level_from: usize,
    ///Amount of time the level has taken
    pub level_no_of_moves: i32,
}
impl Default for GameWinState {
    fn default() -> Self {
        GameWinState {
            ws: WinStateEnum::default(),
            level_from: 0,
            level_no_of_moves: 0,
        }
    }
}
impl GameWinState {
    ///Constructor for GameState with custom arguments
    ///
    ///  - **won_opt** is an option for whether or not the game has been won. If it is None, the game is still being played, or is being started, and if it is Some, then whether the game has been won is the bool
    ///  - **level_from** is for the level that won_opt refers to
    ///  - **level_timer_len** refers to how long the level took
    pub fn new(won_opt: Option<bool>, level_from: usize, level_timer_len: i32) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
                level_from,
                level_no_of_moves: level_timer_len,
            },
            Some(won) => Self {
                ws: WinStateEnum::End { won },
                level_from,
                level_no_of_moves: level_timer_len,
            },
        }
    }
    ///Exactly the same as *GameWinState::new*, but the won_opt is a reference to a boolean
    #[allow(dead_code)]
    pub fn new_ref(won_opt: Option<&bool>, level_from: usize) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
                level_from,
                level_no_of_moves: 0,
            },
            Some(won_ref) => {
                let mut won = false;
                if won_ref == &true {
                    won = true;
                }
                Self {
                    ws: WinStateEnum::End { won },
                    level_from,
                    level_no_of_moves: 0,
                }
            }
        }
    }
}
