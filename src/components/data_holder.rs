use amethyst::core::ecs::Entity;

///Resource to hold all entity related data
#[derive(Default)]
pub struct EntityHolder {
    ///Vector of players
    pub players: Vec<Entity>,

    ///Vector of tiles
    pub tiles: Vec<Entity>,
}
impl EntityHolder {
    ///Constructor to initialise all lists with empty hashmaps/vectors
    pub fn new() -> Self {
        Self::default()
    }

    ///Add a player to the entity list
    pub fn add_player_entity(&mut self, player: Entity) {
        self.players.push(player);
    }

    ///Add a non-actor to the list
    pub fn add_tile(&mut self, e: Entity) {
        self.tiles.push(e);
    }

    ///Gets a list of ALL Entities
    pub fn get_all_entities(&self) -> Vec<Entity> {
        let mut list = Vec::new();
        self.tiles.iter().for_each(|e| list.push(*e));
        self.players.iter().for_each(|e| list.push(*e));

        list
    }
}

#[cfg(test)]
mod data_holder_tests {
    use super::*;
    use amethyst::core::ecs::{World, WorldExt, Builder};

    #[test]
    pub fn data_holder_test () {
        let mut eh = EntityHolder::new();
        let mut world = World::new();
        let mut r = || world.create_entity().build();

        let mut players = vec![r(), r(), r(), r(), r(), r()];
        let mut tiles = vec![r(), r(), r(), r(), r(), r()];

        players.iter().for_each(|e| eh.add_player_entity(e.clone()));
        tiles.iter().for_each(|e| eh.add_tile(e.clone()));

        assert_eq!(&players.sort(), &eh.players.sort());
        assert_eq!(&tiles.sort(), &eh.tiles.sort());

        let mut all = {
            let mut v = Vec::new();
            players.iter().for_each(|e| v.push(e.clone()));
            tiles.iter().for_each(|e| v.push(e.clone()));
            v
        };
        assert_eq!(all.sort(), eh.get_all_entities().sort());
    }
}