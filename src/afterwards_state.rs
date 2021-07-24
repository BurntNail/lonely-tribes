use amethyst::{SimpleState, StateData, GameData};
use crate::components::GameWinState;
use amethyst::core::ecs::WorldExt;

pub struct PostGameState;

impl SimpleState for PostGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let gws = world.read_resource::<GameWinState>();
        log::info!("Post Game boiiii - {:?}", gws.ws);
    }
}