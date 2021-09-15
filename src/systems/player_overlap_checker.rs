use amethyst::core::ecs::{System, Entity, WriteStorage, ReadStorage, Join, Entities, Write};
use crate::components::player::Player;
use crate::components::tile_transform::TileTransform;
use std::collections::HashMap;
use crate::components::win_state::{GameState, GameStateEnum};

pub struct PlayerOverlapChecker;

impl <'s> System<'s> for PlayerOverlapChecker {
    type SystemData = (WriteStorage<'s, Player>, ReadStorage<'s, TileTransform>, Entities<'s>, Write<'s, DeleteList>, Write<'s, GameState>);

    fn run(&mut self, (mut players, tiles, entities, mut delete_list, mut gs): Self::SystemData) {
        let mut map: HashMap<TileTransform, Player> = HashMap::new();
        let mut id_list = Vec::new();
        for (e, p, t) in (&entities, &mut players, &tiles).join() {
            let to_insert = if let Some(mut current) = map.remove(t) {
                if current.id == p.id {
                    current.no_players += 1;
                    delete_list.0.push(e);
                } else {
                    gs.ws = GameStateEnum::End {lost_position: Some(*t)};
                }
                current
            } else {
                *p
            };

            if !id_list.contains(&p.id) {
                id_list.push(p.id);
            }

            map.insert(*t, to_insert);
        }

        if map.len() == id_list.len() {
            gs.ws = GameStateEnum::End {lost_position: None};
        }
    }
}

#[derive(Default)]
pub struct DeleteList (pub Vec<Entity>);