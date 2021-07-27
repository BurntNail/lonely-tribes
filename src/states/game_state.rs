use crate::{
    components::{Collider, ColliderList, GameWinState, TileTransform, WinStateEnum, NPC},
    level::Room,
    states::{
        states_util::{get_trans_puzzle, init_camera, load_sprite_sheet},
        {PostGameState, TrueEnd},
    },
    systems::UpdateTileTransforms,
    tag::Tag,
    {ARENA_HEIGHT, ARENA_WIDTH},
};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    input::VirtualKeyCode,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};
use std::{
    collections::HashMap,
};

lazy_static! {
    ///List of strings holding the file paths to all levels
    pub static ref LEVELS: Vec<String> = {
        vec!["lvl-01.png".to_string(), "lvl-02.png".to_string(), "lvl-03.png".to_string()]
    };
}

///State for when the User is in a puzzle
pub struct PuzzleState {
    ///Holding the current WinState
    ws: WinStateEnum,
    ///The index of the current level in *LEVELS*
    level_index: usize,
    ///Holding a HashMap of which keys lead to which indicies of *LEVELS*
    actions: HashMap<VirtualKeyCode, usize>,
}
impl Default for PuzzleState {
    fn default() -> Self {
        Self {
            ws: WinStateEnum::default(),
            level_index: 0,
            actions: HashMap::new(),
        }
    }
}
impl PuzzleState {
    ///Constructor for PuzzleState
    ///
    ///  - **level_index** is the level index for *LEVELS*
    pub fn new(level_index: usize) -> Self {
        PuzzleState {
            level_index,
            ..Default::default()
        }
    }
}

impl SimpleState for PuzzleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        init_camera(world, (ARENA_WIDTH as f32, ARENA_HEIGHT as f32));

        let handle = load_sprite_sheet(world, "colored_tilemap_packed");

        world.register::<crate::components::NPC>();
        world.insert(GameWinState::new(None, self.level_index, 0));

        let this_level = LEVELS
            .get(self.level_index)
            .unwrap_or(&"test-room-one.png".to_string())
            .as_str()
            .to_string();
        load_level(world, handle, this_level);

        self.actions.insert(VirtualKeyCode::R, self.level_index);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.delete_all();
        log::info!("Deleted all entities");

        if let WinStateEnum::End { won } = self.ws {
            world.insert(GameWinState::new(Some(won), self.level_index, get_no_of_moves(world)));
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        get_trans_puzzle(event, &self.actions)
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let game_state = data.world.read_resource::<GameWinState>();
        let ws = game_state.ws;
        self.ws = ws;

        match ws {
            WinStateEnum::End { won } => {
                if self.level_index >= LEVELS.len() - 1 && won {
                    log::info!("Switching to true end at {}", self.level_index);
                    Trans::Switch(Box::new(TrueEnd::default()))
                } else {
                    log::info!("PGS at {}", self.level_index);
                    Trans::Switch(Box::new(PostGameState::new()))
                }
            }
            WinStateEnum::TBD => Trans::None,
        }
    }
}

///Function to get the number of moves from the last round
fn get_no_of_moves(world: &World) -> i32 {
    let gws = world.read_resource::<GameWinState>();
    gws.level_no_of_moves
}

///Loads in a level given a path
///
///  - **world** is the current game World from Specs
///  - **sprites_handle** is a handle to the spritesheet
///  - **path** is the Path to the level eg. *"lvl-01.png"*
fn load_level(world: &mut World, sprites_handle: Handle<SpriteSheet>, path: String) {
    let lvl = Room::new(path.as_str());

    if lvl.data.is_empty() {
        return;
    }

    for x in 0..lvl.data.len() {
        for y in 0..lvl.data[0].len() {
            let spr_index = lvl.data[x][y].get_spritesheet_index();

            if spr_index == 9999 {
                continue;
            }

            let spr = SpriteRender::new(sprites_handle.clone(), spr_index);
            let tag = Tag::from_spr(lvl.data[x][y]);
            let tt = TileTransform::new(x as i32, y as i32);

            world.insert(ColliderList::new());
            world.insert(GameWinState::default());

            match tag {
                Tag::Player(id) => {
                    let mut trans = Transform::default();
                    trans.set_translation_z(0.5);
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(Collider::new(id))
                        .with(crate::components::Player::new(id))
                        .build();
                }
                Tag::NPC { is_enemy } => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default())
                        .with(NPC::new(is_enemy))
                        .with(Collider::default())
                        .build();
                }
                Tag::Collision => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default()) //TODO: Work out way to optimise for static obj
                        .with(Collider::default())
                        .build();
                }
                Tag::Trigger(trigger_type) => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default())
                        .with(Collider::new(trigger_type.get_id()))
                        .build();
                }
                _ => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(UpdateTileTransforms::tile_to_transform(tt))
                        .build();
                }
            }
        }
    }
}
