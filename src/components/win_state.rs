///Enumeration for the current state of the game
#[derive(Debug, Copy, Clone)]
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
}
impl Default for GameWinState {
    fn default() -> Self {
        GameWinState {
            ws: WinStateEnum::default(),
            level_from: 0,
        }
    }
}
impl GameWinState {
    ///Constructor for GameState with custom arguments
    ///
    ///  - **won_opt** is an option for whether or not the game has been won. If it is None, the game is still being played, or is being started, and if it is Some, then whether the game has been won is the bool
    ///  - **level_from** is for the level that won_opt refers to
    pub fn new(won_opt: Option<bool>, level_from: usize) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
                level_from,
            },
            Some(won) => Self {
                ws: WinStateEnum::End { won },
                level_from,
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
            },
            Some(won_ref) => {
                let mut won = false;
                if won_ref == &true {
                    won = true;
                }
                Self {
                    ws: WinStateEnum::End { won },
                    level_from,
                }
            }
        }
    }
}
