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
#[derive(Copy, Clone, Debug)]
pub struct GameWinState {
    pub ws: WinStateEnum,
}
impl Default for GameWinState {
    fn default() -> Self {
        GameWinState {
            ws: WinStateEnum::default(),
        }
    }
}
impl GameWinState {
    pub fn new(won_opt: Option<bool>) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
            },
            Some(won) => Self {
                ws: WinStateEnum::End { won },
            },
        }
    }
    #[allow(dead_code)]
    pub fn new_ref(won_opt: Option<&bool>) -> Self {
        match won_opt {
            None => Self {
                ws: WinStateEnum::TBD,
            },
            Some(won_ref) => {
                let mut won = false;
                if won_ref == &true {
                    won = true;
                }
                Self {
                    ws: WinStateEnum::End { won },
                }
            }
        }
    }
}
