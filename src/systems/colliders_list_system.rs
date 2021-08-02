use crate::{
    components::{
        colliders::{Collider, ColliderList},
        player::Player,
        tile_transform::TileTransform,
        win_state::GameWinState,
    },
    quick_save_load::LevelState,
    tag::TriggerType,
};
use amethyst::{
    core::ecs::Read,
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
        Read<'s, GameWinState>,
        Write<'s, LevelState>,
    );

    fn run(&mut self, (tiles, colliders, players, mut list, gws, mut level): Self::SystemData) {
        let mut colliders_list = Vec::new();
        let mut triggers_list = Vec::new();

        let mut power_list = Vec::new();
        let mut player_list = Vec::new();

        for (t, c) in (&tiles, &colliders).join() {
            let tt = TileTransform::from_ref(t);
            if let Some(t) = c.trigger {
                triggers_list.push((tt, t));
                if let TriggerType::Powerup(p) = t {
                    power_list.push((p, tt));
                }
            } else {
                colliders_list.push(tt);
            }
        }

        (&tiles, &players)
            .join()
            .for_each(|(t, p)| player_list.push((*p, *t)));

        list.set(colliders_list);
        list.set_triggers(triggers_list);

        level.replace(player_list, power_list, gws.level_no_of_moves);
    }
}
