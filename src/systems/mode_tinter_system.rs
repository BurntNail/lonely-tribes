use crate::components::win_state::GameModeManager;
use amethyst::{
    core::ecs::{Join, Read, System, WriteStorage},
    renderer::resources::Tint,
};

///System to change tint on FX entities dependent on how many moves are left
pub struct GameModeTinterSystem;

impl<'s> System<'s> for GameModeTinterSystem {
    type SystemData = (Read<'s, GameModeManager>, WriteStorage<'s, Tint>);

    fn run(&mut self, (gm, mut tints): Self::SystemData) {
        let t = ((1.0 - gm.moves_left as f32 / gm.total_moves as f32) * 10.0).log10();

        for tint in (&mut tints).join() {
            if (tint.0.blue - 1.0).abs() < f32::EPSILON {
                //replaced "tint.0.blue == 1.0" with this to avoid float cmp errors
                //this is the blue one
                tint.0.red = t;
            } else {
                //this is the red one, so the r is 100%, and we change the g and the b
                tint.0.blue = t;
            }
            tint.0.green = t;
        }
    }
}
