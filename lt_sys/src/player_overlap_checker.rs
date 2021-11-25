use amethyst::core::ecs::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage};
use lonely_tribes_components::{
    player::Player,
    point_light::PointLight,
    tile_transform::TileTransform,
    win_related::{GameState, GameStateEnum},
};
use std::collections::HashMap;

pub struct PlayerOverlapChecker;

impl<'s> System<'s> for PlayerOverlapChecker {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, PointLight>,
        ReadStorage<'s, TileTransform>,
        Entities<'s>,
        Write<'s, DeleteList>,
        Write<'s, GameState>,
    );

    fn run(
        &mut self,
        (mut players, mut lights, tiles, entities, mut delete_list, mut gs): Self::SystemData,
    ) {
        if gs.ws == GameStateEnum::ToBeDecided {
            let mut map: HashMap<TileTransform, &mut Player> = HashMap::new();
            let mut id_list = Vec::new();
            let mut lost = false;

            for (e, p, t) in (&entities, &mut players, &tiles).join() {
                let p_id = p.id;
                let to_insert = if let Some(mut current) = map.remove(t) {
                    if current.id == p_id {
                        current.no_players += 1;
                        delete_list.0.push(e);
                    } else {
                        gs.ws = GameStateEnum::End {
                            lost_position: Some(*t),
                        };
                        lost = true;
                        break;
                    }
                    current
                } else {
                    p
                };

                if !id_list.contains(&p_id) {
                    id_list.push(p_id);
                }

                map.insert(*t, to_insert);
            }

            if map.len() == id_list.len() && !lost {
                gs.ws = GameStateEnum::End {
                    lost_position: None,
                };
            }
        }

        for (p, l) in (&players, &mut lights).join() {
            let r = ((p.no_players + 1) as f32 * 3.0) as u32;
            l.radius = r;
        }
    }
}

#[derive(Default)]
pub struct DeleteList(pub Vec<Entity>);
