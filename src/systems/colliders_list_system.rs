use amethyst::{
    core::{transform::Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use crate::level::Room;
use crate::components::{TileTransform, Collider, ColliderList};
use crate::HEIGHT;

#[derive(SystemDesc)]
pub struct CollidersListSystem;

impl<'s> System<'s> for CollidersListSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, ColliderList>
    );

    fn run(&mut self, (tiles, colliders, mut lists): Self::SystemData) {
        let mut colliders_list = Vec::new();
        for (t, _col) in (&tiles, &colliders).join() {
            colliders_list.push(TileTransform::from_ref(t));
        }
        for list in (&mut lists).join() {
            list.set(colliders_list.clone())
        }
    }

}