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
        Self {
            ..Default::default()
        }
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
