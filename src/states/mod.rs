mod afterwards_state;
mod game_state;
mod states_util;
mod welcome_state;
mod true_end;

pub use afterwards_state::PostGameState;
pub use game_state::{PuzzleState, LEVELS};
pub use welcome_state::StartGameState;
pub use true_end::TrueEnd;