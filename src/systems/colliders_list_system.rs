use crate::{
    components::{
        colliders::{Collider, ColliderList},
        player::Player,
        tile_transform::TileTransform,
    },
};
use amethyst::{
    ecs::{Join, ReadStorage, System, Write},
};

///System to update ColliderList, and LevelState
pub struct ListSystem;

impl<'s> System<'s> for ListSystem {
    type SystemData = (
        ReadStorage<'s, TileTransform>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Player>,
        Write<'s, ColliderList>,
    );

    fn run(&mut self, (tiles, colliders, players, mut list): Self::SystemData) {
        let mut colliders_list = Vec::new();
        let mut triggers_list = Vec::new();
        let mut player_list = Vec::new();

        for (t, c) in (&tiles, &colliders).join() {
            let tt = *t;
            if let Some(t) = c.trigger {
                triggers_list.push((tt, t));
            } else {
                colliders_list.push(tt);
            }
        }

        (&tiles, &players)
            .join()
            .for_each(|(t, p)| player_list.push((*p, *t)));

        list.set(colliders_list);
        list.set_triggers(triggers_list);
    }
}
