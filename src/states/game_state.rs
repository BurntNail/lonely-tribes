use crate::{
    components::{
        animator::Animator,
        colliders::{Collider, ColliderList},
        data_holder::EntityHolder,
        npc::NonPlayerCharacter,
        score::Score,
        tile_transform::TileTransform,
        win_state::{GameWinState, WinStateEnum},
    },
    level::{Room, SpriteRequest},
    quick_save_load::{LevelState, SaveType},
    states::{
        afterwards_state::PostGameState,
        level_select::LevelSelectState,
        states_util::{init_camera, load_font, load_sprite_sheet},
        true_end::TrueEnd,
    },
    systems::update_tile_transforms::UpdateTileTransforms,
    tag::{Tag, TriggerType},
    Flags, ARENA_HEIGHT, ARENA_WIDTH,
};
use amethyst::{
    assets::Handle,
    core::{ecs::Entity, transform::Transform, Hidden},
    input::{InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, Interactable, LineMode, UiText, UiTransform},
    winit::{Event, WindowEvent},
};
use std::collections::HashMap;
use structopt::StructOpt;

lazy_static! {
    ///List of strings holding the file paths to all levels
    pub static ref LEVELS: Vec<String> = {
        get_levels()
    };
}
pub fn get_levels() -> Vec<String> {
    let opts: Flags = Flags::from_args();

    if cfg!(debug_assertions) && opts.debug && opts.debug_level {
        vec!["test-room-one.png".to_string()]
    } else {
        let mut out: Vec<String> = LevelState::list_file_names_in_dir("assets/maps")
            .into_iter()
            .filter(|nom| nom.contains("lvl-") && nom.contains(".png"))
            .map(|el| el.replace("\"", ""))
            .collect();
        out.sort();
        out
    }
}

///State for when the User is in a puzzle
pub struct PuzzleState {
    ///Holding the current WinState
    ws: WinStateEnum,
    ///The index of the current level in *LEVELS*
    level_index: usize,
    ///Holding a HashMap of which keys lead to which indicies of *LEVELS*
    actions: HashMap<VirtualKeyCode, usize>,
    ///Option variable to hold the Score text
    score_button: Option<Entity>,
    ///Option variable to hold loaded level
    level_state: Option<LevelState>,
}
impl Default for PuzzleState {
    fn default() -> Self {
        let opts = Flags::from_args();

        let mut level_index = 0;
        if opts.debug {
            level_index = opts.level.unwrap_or(1) - 1;
            if level_index > LEVELS.len() - 1 {
                level_index = 0;
            }
            log::info!("Starting early at level {}", level_index);
        }

        Self {
            ws: WinStateEnum::default(),
            level_index,
            level_state: None,
            actions: HashMap::new(),
            score_button: None,
        }
    }
}
impl PuzzleState {
    ///Constructor for PuzzleState
    pub fn new(level_index: usize) -> Self {
        PuzzleState {
            level_index,
            ..Default::default()
        }
    }
    ///Constructor for PuzzleState for use with a loaded save
    pub fn new_levelstate(level_index: usize, level_state: Option<LevelState>) -> Self {
        PuzzleState {
            level_index,
            level_state,
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

        let this_level = LEVELS
            .get(self.level_index)
            .unwrap_or(&"test-room-one.png".to_string())
            .as_str()
            .to_string();

        let holder = if let Some(state) = self.level_state.clone() {
            world.insert(GameWinState::new(None, self.level_index, state.score));
            load_level_with_(world, handle, this_level, state)
        } else {
            world.insert(GameWinState::new(None, self.level_index, 0));
            load_level(world, handle, this_level)
        };

        world.register::<NonPlayerCharacter>();

        world.insert(holder);
        world.insert(LevelState::default());

        self.actions.insert(VirtualKeyCode::R, self.level_index);

        self.score_button = Some(add_score(world));
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.delete_all();

        if let WinStateEnum::End { won } = self.ws {
            world.insert(GameWinState::new(
                Some(won),
                self.level_index,
                get_no_of_moves(world),
            ));
        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = Trans::None;
        use VirtualKeyCode::*;

        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => match key_code {
                Space => {
                    if let Some(btn) = self.score_button {
                        let mut hiddens = data.world.write_storage::<Hidden>();
                        if hiddens.contains(btn) {
                            hiddens.remove(btn);
                        } else {
                            hiddens.insert(btn, Hidden).unwrap_or_else(|e| {
                                log::error!("Unable to insert btn into hiddens - {}", e);
                                None
                            });
                        }
                    }
                }
                L | Escape => t = Trans::Switch(Box::new(LevelSelectState::default())),
                F5 | VirtualKeyCode::F6 => data
                    .world
                    .read_resource::<LevelState>()
                    .save(SaveType::QuickSave, self.level_index),
                F9 => {
                    let save = LevelState::load_most_recent(None, self.level_index);
                    t = Trans::Switch(Box::new(PuzzleState::new_levelstate(
                        self.level_index,
                        save,
                    )));
                }

                _ => self.actions.iter().for_each(|(k, v)| {
                    if &key_code == k {
                        t = Trans::Switch(Box::new(PuzzleState::new(*v)));
                    }
                }),
            },
            StateEvent::Window(Event::WindowEvent { event, .. }) => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    let mut gws = data.world.write_resource::<GameWinState>();
                    gws.ws = WinStateEnum::End { won: false };
                }
                _ => {}
            },
            _ => {}
        };

        t
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let game_state = data.world.read_resource::<GameWinState>();
        let ws = game_state.ws;
        self.ws = ws;

        match ws {
            WinStateEnum::End { won } => {
                if self.level_index >= LEVELS.len() - 1 && won {
                    Trans::Switch(Box::new(TrueEnd::default()))
                } else {
                    Trans::Switch(Box::new(PostGameState::new()))
                }
            }
            WinStateEnum::ToBeDecided => Trans::None,
        }
    }
}

///Adds an entity with UiText to mark the score to the player
fn add_score(world: &mut World) -> Entity {
    let trans = UiTransform::new(
        "score_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        -575.0,
        400.0,
        0.5,
        350.0,
        1000.0,
    );
    let txt = UiText::new(
        load_font(world, "ZxSpectrumBold"),
        "Current Score: 0".to_string(),
        [1.0, 1.0, 1.0, 0.5],
        25.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world
        .create_entity()
        .with(trans)
        .with(txt)
        .with(Interactable)
        .with(Score)
        .build()
}

///Function to get the number of moves from this round
fn get_no_of_moves(world: &World) -> i32 {
    let gws = world.read_resource::<GameWinState>();
    gws.level_no_of_moves
}

///Loads in a level given a path
///
///  - **world** is the current game World from Specs
///  - **sprites_handle** is a handle to the spritesheet
///  - **path** is the Path to the level eg. *"lvl-01.png"*
fn load_level(
    world: &mut World,
    sprites_handle: Handle<SpriteSheet>,
    path: String,
) -> EntityHolder {
    let lvl = Room::new(path.as_str());
    let mut holder = EntityHolder::new();

    if lvl.is_empty() {
        return holder;
    }

    for x in 0..lvl.len() {
        for y in 0..lvl[0].len() {
            let spr_index = lvl[x][y].get_spritesheet_index();

            if spr_index == 9999 {
                continue;
            }

            let spr = SpriteRender::new(sprites_handle.clone(), spr_index);
            let tt = TileTransform::new(x as i32, y as i32);

            world.insert(ColliderList::new());
            world.insert(GameWinState::default());

            match Tag::from_spr(lvl[x][y]) {
                Tag::Player(id) => {
                    let mut trans = Transform::default();
                    trans.set_translation_z(0.5);
                    let ent = world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(Collider::new(TriggerType::from_id(&id)))
                        .with(crate::components::player::Player::new(id))
                        .with(Animator::new())
                        .build();
                    holder.add_entity(ent);
                }
                Tag::NonPlayerCharacter { is_enemy } => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default())
                        .with(NonPlayerCharacter::new(is_enemy))
                        .with(Collider::default())
                        .build();
                }
                Tag::Collision => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default())
                        .with(Collider::default())
                        .build();
                }
                Tag::Trigger(trigger_type) => match trigger_type {
                    TriggerType::Powerup(_) => {
                        let e = world
                            .create_entity()
                            .with(spr)
                            .with(tt)
                            .with(Transform::default())
                            .with(Collider::new(trigger_type))
                            .build();
                        holder.add_pu_entity(tt, e);
                    }
                    _ => {
                        world
                            .create_entity()
                            .with(spr)
                            .with(tt)
                            .with(Transform::default())
                            .with(Collider::new(trigger_type))
                            .build();
                    }
                },
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

    holder
}

///Loads in a level given a path and a levelState
fn load_level_with_(
    world: &mut World,
    sprites_handle: Handle<SpriteSheet>,
    path: String,
    level: LevelState,
) -> EntityHolder {
    let lvl = Room::new(path.as_str());
    let mut holder = EntityHolder::new();

    level.players.into_iter().for_each(|(p, tt)| {
        let mut trans = Transform::default();
        trans.set_translation_z(0.5);
        let ent = world
            .create_entity()
            .with(SpriteRender::new(
                sprites_handle.clone(),
                SpriteRequest::Player(p.id).get_spritesheet_index(),
            ))
            .with(tt)
            .with(trans)
            .with(Collider::new(TriggerType::from_id(&p.id)))
            .with(p)
            .with(Animator::new())
            .build();
        holder.add_entity(ent);
    });
    level.powerups.into_iter().for_each(|(p, tt)| {
        let e = world
            .create_entity()
            .with(SpriteRender::new(
                sprites_handle.clone(),
                SpriteRequest::PowerUpSprite(p).get_spritesheet_index(),
            ))
            .with(tt)
            .with(Transform::default())
            .with(Collider::new(TriggerType::Powerup(p)))
            .build();
        holder.add_pu_entity(tt, e);
    });

    if lvl.is_empty() {
        return holder;
    }

    for x in 0..lvl.len() {
        for y in 0..lvl[0].len() {
            let spr_index = lvl[x][y].get_spritesheet_index();

            if spr_index == 9999 {
                continue;
            }

            let spr = SpriteRender::new(sprites_handle.clone(), spr_index);
            let tag = Tag::from_spr(lvl[x][y]);
            let tt = TileTransform::new(x as i32, y as i32);

            match tag {
                Tag::Player(_) => {
                    //do nothing because they get created earlier
                }
                Tag::NonPlayerCharacter { is_enemy } => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default())
                        .with(NonPlayerCharacter::new(is_enemy))
                        .with(Collider::default())
                        .build();
                }
                Tag::Collision => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(Transform::default())
                        .with(Collider::default())
                        .build();
                }
                Tag::Trigger(trigger_type) => match trigger_type {
                    TriggerType::Powerup(_) => {
                        //do nothing because they get created earlier
                    }
                    _ => {
                        world
                            .create_entity()
                            .with(spr)
                            .with(tt)
                            .with(Transform::default())
                            .with(Collider::new(trigger_type))
                            .build();
                    }
                },
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

    holder
}
