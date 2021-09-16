use crate::components::{
    player::Player,
    tile_transform::TileTransform,
    win_state::{GameState, GameStateEnum},
};
use amethyst::core::ecs::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;
use crate::components::point_light::PointLight;

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

    fn run(&mut self, (mut players, mut lights, tiles, entities, mut delete_list, mut gs): Self::SystemData) {
        let mut map: HashMap<TileTransform, &mut Player> = HashMap::new();
        let mut id_list = Vec::new();
        for (e, p, t) in (&entities, &mut players, &tiles).join() {
            let p_id = p.id;
            let to_insert = if let Some(mut current) = map.remove(t) {
                if current.id == p_id {
                    log::info!("found");
                    current.no_players += 1;
                    delete_list.0.push(e);
                } else {
                    gs.ws = GameStateEnum::End {
                        lost_position: Some(*t),
                    };
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

        if map.len() == id_list.len() {
            gs.ws = GameStateEnum::End {
                lost_position: None,
            };
        }

        for (p, l) in (&players, &mut lights).join() { //TODO: farm out to different sys
            let r = ((p.no_players + 1) as f32 * 3.0) as u32; //TODO: work out better func for rad, maybe bands (eg. 1 = 3, 2-3 = 5, 3-5 = 6, 5-8 = 9)
            log::info!("new rad is {} for {}", r, p.no_players + 1);
            l.radius = r;
        }
    }
}

#[derive(Default)]
pub struct DeleteList(pub Vec<Entity>);
