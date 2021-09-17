use amethyst::core::ecs::{System, Read};
use amethyst::utils::fps_counter::FpsCounter;

pub struct FpsPrinterSystem;

impl<'s> System<'s> for FpsPrinterSystem {
    type SystemData = Read<'s, FpsCounter>;

    fn run(&mut self, fps: Self::SystemData) {
        log::info!("FPS: {}", fps.frame_fps())
    }
}
