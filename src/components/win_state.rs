#[derive(Debug, Copy, Clone)]
pub enum WinStateEnum {
    End { won: bool },
    TBD,
}
impl Default for WinStateEnum {
    fn default() -> Self {
        Self::TBD
    }
}
#[derive(Clone, Debug)]
pub struct GameWinState {
    pub ws: WinStateEnum,
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
