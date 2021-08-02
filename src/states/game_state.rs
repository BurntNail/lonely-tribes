use crate::{
    components::{
        Collider, ColliderList, GameWinState, NonPlayerCharacter, PowerUpHolder, Score,
        TileTransform, WinStateEnum,
    },
    level::Room,
    states::{
        level_select::LevelSelectState,
        states_util::{init_camera, load_font, load_sprite_sheet},
        PostGameState, TrueEnd,
    },
    systems::UpdateTileTransforms,
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
use crate::quick_save_load::{LevelState, SaveType};

lazy_static! {
    ///List of strings holding the file paths to all levels
    pub static ref LEVELS: Vec<String> = {
        let opts: Flags = Flags::from_args();

        let mut out = (1..=7).into_iter().map(|n| format!("lvl-{:02}.png", n)).collect();

        if cfg!(debug_assertions) && opts.debug && opts.debug_level{
            out = vec!["test-room-one.png".to_string()];
        }

        out
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
    ///Option variable to hold the Score text
    score_button: Option<Entity>,
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
            actions: HashMap::new(),
            score_button: None,
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

        let this_level = LEVELS
            .get(self.level_index)
            .unwrap_or(&"test-room-one.png".to_string())
            .as_str()
            .to_string();
        let holder = load_level(world, handle, this_level);

        world.register::<crate::components::NonPlayerCharacter>();
        world.insert(GameWinState::new(None, self.level_index, 0));
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

        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => {
                match key_code {
                    VirtualKeyCode::Space => {
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
                    },
                    VirtualKeyCode::L => t = Trans::Switch(Box::new(LevelSelectState::default())),
                    VirtualKeyCode::F5 | VirtualKeyCode::F6 => data.world.read_resource::<LevelState>().save(SaveType::QuickSave, self.level_index),
                    VirtualKeyCode::F9 => {
                        let save = LevelState::load_most_recent(None, self.level_index);
                        log::info!("{:?}", save.unwrap_or_default());
                        //TODO: make new constructor that takes in LevelState
                        //TODO: new load_level with LevelState option
                    }

                    _ => self.actions.iter().for_each(|(k, v)| {
                        if &key_code == k {
                            t = Trans::Switch(Box::new(PuzzleState::new(*v)));
                        }
                    })
                }
            }
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
) -> PowerUpHolder {
    let lvl = Room::new(path.as_str()); //TODO: Just use the map straight away
    let mut holder = PowerUpHolder::new();

    if lvl.data.is_empty() {
        return holder;
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
                    let ent = world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(Collider::new(TriggerType::from_id(&id)))
                        .with(crate::components::Player::new(id))
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
                        .with(Transform::default()) //TODO: Work out way to optimise for static obj
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
