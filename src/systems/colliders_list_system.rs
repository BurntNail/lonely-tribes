use crate::components::{Collider, ColliderList, TileTransform};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, Write},
};

#[derive(SystemDesc)]
pub struct CollidersListSystem;

impl<'s> System<'s> for CollidersListSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, Collider>,
        Write<'s, ColliderList>,
    );

    fn run(&mut self, (tiles, colliders, mut list): Self::SystemData) {
        let mut colliders_list = Vec::new();
        let mut triggers_list = Vec::new();

        for (t, c) in (&tiles, &colliders).join() {
            let tt = TileTransform::from_ref(t);
            if c.trigger.is_some() {
                triggers_list.push((tt, c.trigger.unwrap_or(9999)));
            } else {
                colliders_list.push(tt);
            }
        }

        list.set(colliders_list);
        list.set_triggers(triggers_list);
    }
}
