use amethyst::{
    core::math::Point3,
    ecs::World,
    tiles::Tile,
};

#[derive(Clone, Default)]
pub struct SimpleTile;
impl Tile for SimpleTile {
    fn sprite(&self, _coords: Point3<u32>, _: &World) -> Option<usize> {
        Some(1)
    }
}