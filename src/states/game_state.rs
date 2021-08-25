use crate::{
    components::{
        animator::Animator,
        colliders::{Collider, ColliderList},
        data_holder::EntityHolder,
        npc::NonPlayerCharacter,
        score::Score,
        tile_transform::TileTransform,
        win_state::{GameModeManager, GamePlayingMode, GameState, GameStateEnum},
    },
    file_utils::list_file_names_in_dir,
    level::Room,
    states::{
        afterwards_state::PostGameState,
        level_select::LevelSelectState,
        paused_state::PausedState,
        states_util::{init_camera, load_font, load_sprite_sheet},
        true_end::TrueEnd,
    },
    systems::update_tile_transforms::UpdateTileTransforms,
    tag::{Tag, TriggerType},
    Flags, ARENA_HEIGHT, ARENA_WIDTH,
};
use amethyst::{
    assets::Handle,
    core::{ecs::Entity, transform::Transform, Hidden, Time},
    input::{InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet},
    ui::{Anchor, Interactable, LineMode, UiText, UiTransform},
    winit::{Event, WindowEvent},
};
use std::collections::HashMap;
use structopt::StructOpt;
use crate::components::animator::AnimInterpolation;
use amethyst::core::math::Vector3;

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
        let mut out: Vec<String> = list_file_names_in_dir("assets/maps")
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
    ws: GameStateEnum,
    ///The index of the current level in *LEVELS*
    level_index: usize,
    ///Holding a HashMap of which keys lead to which indicies of *LEVELS*
    actions: HashMap<VirtualKeyCode, usize>,
    ///Option variable to hold the Score text
    score_button: Option<Entity>,
    ///Vec to hold entities for temporary mode effects (eg. nudger)
    tmp_fx_entities: Vec<Entity>,
    ///timer for when we lose containing (so far, duration, entity)
    death_timer: Option<(f32, f32, Entity)>,
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
            ws: GameStateEnum::default(),
            level_index,
            actions: HashMap::new(),
            score_button: None,
            tmp_fx_entities: Vec::new(),
            death_timer: None,
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

    ///Sets the mode to normal, and deletes all the fx entities
    pub fn reset_fx_entities(&mut self, world: &mut World) {
        if self.tmp_fx_entities.is_empty() {
            return;
        }
        log::info!("reseto spaghetto");

        for e in std::mem::take(&mut self.tmp_fx_entities) {
            world
                .delete_entity(e)
                .unwrap_or_else(|err| log::warn!("Unable to delete tmp fx entitity: {}", err));
        }
    }
}

impl SimpleState for PuzzleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        init_camera(world, (ARENA_WIDTH as f32, ARENA_HEIGHT as f32));

        let handle = load_sprite_sheet(world, "colored_tilemap_packed");

        let level_default = "test-room-one.png".to_string();
        let this_level = {
            let this_level = LEVELS.get(self.level_index).unwrap_or(&level_default);
            this_level.as_str()
        };

        world.insert(GameState::new(None, self.level_index, 0));
        let holder = load_level(world, handle, this_level);

        world.register::<NonPlayerCharacter>();

        world.insert(holder);
        world.insert(GameModeManager::new(50));

        self.actions.insert(VirtualKeyCode::R, self.level_index);

        self.score_button = Some(add_score(world));
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.delete_all();

        if let GameStateEnum::End { lost_position } = self.ws {
            world.insert(GameState::new(
                Some(lost_position),
                self.level_index,
                get_no_of_moves(world),
            ));
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(btn) = self.score_button {
            data.world.write_storage::<Hidden>().remove(btn);
        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = Trans::None;
        let world = data.world;
        use VirtualKeyCode::*;

        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => match key_code {
                Space => {
                    if let Some(btn) = self.score_button {
                        let mut hiddens = world.write_storage::<Hidden>();
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
                L => t = Trans::Switch(Box::new(LevelSelectState::default())),
                Escape => {
                    if let Some(btn) = self.score_button {
                        world
                            .write_storage::<Hidden>()
                            .insert(btn, Hidden)
                            .unwrap_or_else(|err| {
                                log::warn!("Error hiding things for pausing: {}", err);
                                None
                            });
                    }

                    t = Trans::Push(Box::new(PausedState::default()));
                }
                N => self.set_gameplay_mode(GamePlayingMode::Nudger, world),
                T => self.set_gameplay_mode(GamePlayingMode::TradeOff, world),
                C => self.set_gameplay_mode(GamePlayingMode::Crazy, world),
                K => self.set_gameplay_mode(GamePlayingMode::AllTheColliders, world),
                B => self.set_gameplay_mode(GamePlayingMode::Boring, world),
                _ => self.actions.iter().for_each(|(k, v)| {
                    if &key_code == k {
                        t = Trans::Switch(Box::new(PuzzleState::new(*v)));
                    }
                }),
            },
            StateEvent::Window(Event::WindowEvent { event, .. }) => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    let mut gws = world.write_resource::<GameState>();
                    gws.ws = GameStateEnum::End {
                        lost_position: Some(TileTransform::default()),
                    };
                }
                _ => {}
            },
            _ => {}
        };

        t
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let mut t = Trans::None;

        if data.world.read_resource::<GameModeManager>().current_mode == GamePlayingMode::Boring {
            self.reset_fx_entities(data.world);
        }

        {
            let game_state = data.world.read_resource::<GameState>();
            self.ws = game_state.ws;
        }

        if let GameStateEnum::End { lost_position } = self.ws {
            let won = lost_position.is_none();

            if self.level_index >= LEVELS.len() - 1 && won {
                //we won the last level
                t = Trans::Switch(Box::new(TrueEnd::default()));
            } else if won {
                //we won a level that has another after it
                t = Trans::Switch(Box::new(PostGameState::new()));
            } else {
                //we lost

                let (so_far, total, ent) = self.death_timer.take().unwrap_or_else(|| {
                    let pos = lost_position.unwrap_or_default();
                    let (x, y) = UpdateTileTransforms::tile_to_xyz(pos);

                    let mut trans = Transform::default();
                    trans.set_translation_xyz(x, y, 2.0);

                    let spritesheet = load_sprite_sheet(data.world, "zoom-in-on-loss");

                    let ent = data.world.create_entity().with(trans).with(SpriteRender::new(spritesheet, 0)).build();

                    (0.0, 1.5, ent)
                });

                if so_far > total {
                    //anim is done
                    t = Trans::Switch(Box::new(PostGameState::new()));
                } else {
                    let so_far = so_far + data.world.read_resource::<Time>().delta_seconds();
                    let scale_val = AnimInterpolation::ReverseExponential.get_val_from_pctg(so_far / total) * 4.0 + 1.0;

                    if let Some(trans) = data.world.write_storage::<Transform>().get_mut(ent) {
                        let scale = Vector3::from([scale_val; 3]);
                        trans.set_scale(scale);
                    }

                    self.death_timer = Some((so_far, total, ent));
                }
            }
        }

        t
    }
}

impl PuzzleState {
    fn set_gameplay_mode(&mut self, new_mode: GamePlayingMode, world: &mut World) {
        let can_change = {
            let mut current_mode = world.write_resource::<GameModeManager>();
            if current_mode.current_mode != new_mode {
                current_mode.set_mode(new_mode)
            } else {
                false
            }
        };

        if can_change {
            self.make_fx_entities(world);
        } else {
            world
                .write_resource::<GameModeManager>()
                .set_mode(GamePlayingMode::Boring);
            self.reset_fx_entities(world);
        }
    }

    fn make_fx_entities(&mut self, world: &mut World) {
        let mut entities_to_make = Vec::new();
        {
            let sprite_renderers = world.read_storage::<SpriteRender>();
            let tiletransforms = world.read_storage::<TileTransform>();

            for e in &world.read_resource::<EntityHolder>().tiles {
                if let Some(spr) = sprite_renderers.get(*e) {
                    if let Some(tt) = tiletransforms.get(*e) {
                        let (tt1, tt2) = {
                            // let tw = TILE_WIDTH as i32 / 2;
                            // let th = TILE_HEIGHT as i32 / 2;
                            let tw = 1;
                            let th = 1;

                            let mut tt1 = *tt;
                            let mut tt2 = *tt;

                            tt1.set_offsets((tw, th));
                            tt2.set_offsets((-tw, -th));

                            (tt1, tt2)
                        };

                        let ti1 = Tint(Srgba::new(1.0, 0.0, 0.0, 0.5));
                        let ti2 = Tint(Srgba::new(0.25, 0.0, 1.0, 0.5));

                        let spr = spr.clone();

                        entities_to_make.push((spr.clone(), tt1, ti1));
                        entities_to_make.push((spr.clone(), tt2, ti2));
                    }
                }
            }
        }

        for (s, tt, ti) in entities_to_make {
            let mut trans = Transform::default();

            trans.set_translation_z(0.15);

            let ent = world
                .create_entity()
                .with(s)
                .with(tt)
                .with(ti)
                .with(trans)
                .build();
            self.tmp_fx_entities.push(ent);
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
    let gws = world.read_resource::<GameState>();
    gws.level_no_of_moves
}

///Loads in a level given a path
///
///  - **world** is the current game World from Specs
///  - **sprites_handle** is a handle to the spritesheet
///  - **path** is the Path to the level eg. *"lvl-01.png"*
fn load_level(world: &mut World, sprites_handle: Handle<SpriteSheet>, path: &str) -> EntityHolder {
    let lvl = Room::new(path);
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
            world.insert(GameState::default());

            let mut trans = Transform::default();
            trans.set_translation_z(0.1);

            match Tag::from_spr(lvl[x][y]) {
                Tag::Player(id) => {
                    trans.set_translation_z(0.2);

                    let ent = world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(Collider::new(TriggerType::from_id(&id)))
                        .with(crate::components::player::Player::new(id))
                        .with(Animator::new())
                        .build();
                    holder.add_player_entity(ent);
                }
                Tag::NonPlayerCharacter { is_enemy } => {
                    world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(NonPlayerCharacter::new(is_enemy))
                        .with(Collider::default())
                        .build();
                }
                Tag::Collision => {
                    let ent = world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(Collider::default())
                        .build();
                    holder.add_tile(ent);
                }
                Tag::Trigger(trigger_type) => {
                    let ent = world
                        .create_entity()
                        .with(spr)
                        .with(tt)
                        .with(trans)
                        .with(Collider::new(trigger_type))
                        .build();
                    holder.add_tile(ent);
                }
                _ => {
                    let ent = world
                        .create_entity()
                        .with(spr)
                        .with(UpdateTileTransforms::tile_to_transform(tt, 0.1))
                        .build();
                    holder.add_tile(ent);
                }
            }
        }
    }

    holder
}
