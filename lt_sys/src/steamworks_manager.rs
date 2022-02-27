use amethyst::core::ecs::{System, Write};
use lonely_tribes_lib::SteamworksHolder;

pub struct SteamworksManager;

impl<'s> System<'s> for SteamworksManager {
    type SystemData = (Write<'s, SteamworksHolder>);

    fn run(&mut self, steam: Self::SystemData) {
        steam.1.run_callbacks();
    }
}
