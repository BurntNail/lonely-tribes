use crate::{
    components::{
        animator::{AnimInterpolation, AnimationData, Animator},
        tile_transform::TileTransform,
    },
    level::{Room, SpriteRequest, LIST_OF_ALL_SPRITEREQUESTS, REVERSED_SPRITESHEET_SWATCH_HASHMAP},
    level_editor::editor_select_state::LevelEditorLevelSelectState,
    states::{
        game_state::get_levels,
        states_util::{init_camera, load_sprite_sheet},
    },
    systems::move_player::PLAYER_MOVEMENT_ANIM_LEN,
    ARENA_HEIGHT, ARENA_WIDTH, HEIGHT, WIDTH,
};
use amethyst::{
    assets::Handle,
    core::{
        ecs::{Builder, Entity, World, WorldExt},
        Transform,
    },
    input::{InputEvent, ScrollDirection, VirtualKeyCode},
    renderer::{SpriteRender, SpriteSheet},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};
use image::{ImageBuffer, Rgba};

#[derive(Clone, Default)]
pub struct LevelEditorState {
    data: Room,
    sprite_sheet: Option<Handle<SpriteSheet>>,
    current_editing: TileTransform,
    entities: Vec<Vec<Entity>>,
    highlighter: Option<Entity>,
    saved_state: SpriteRequest,
    level_editing: usize,
}

impl LevelEditorState {
    pub fn new(index: &Option<usize>) -> Self {
        let mut data = Room::default();

        let level_editing;
        if let Some(index) = index {
            level_editing = *index;
            if let Some(lvl) = get_levels().get(*index) {
                data = Room::new(lvl.as_str());
            }
        } else {
            level_editing = get_levels().len();
            let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgba::from([0, 0, 0, 0]));

            let path = format!("lvl-{:02}.png", level_editing + 1);
            img.save(format!("assets/maps/{}", path))
                .unwrap_or_else(|err| log::error!("Error creating new level file: {}", err));

            data = Room::new(path.as_str());
        }

        Self {
            data,
            level_editing,
            ..Default::default()
        }
    }
    pub fn save_self(&self) {
        let mut img = ImageBuffer::new(WIDTH, HEIGHT);
        for (x, y, px) in img.enumerate_pixels_mut() {
            let res = *REVERSED_SPRITESHEET_SWATCH_HASHMAP
                .get(&self.data[x as usize][y as usize])
                .unwrap_or(&Rgba::from([0; 4]));

            *px = res;
        }

        img.save(format!("assets/maps/lvl-{:02}.png", self.level_editing + 1))
            .unwrap_or_else(|err| log::error!("Error saving level in editor: {}", err));
    }
}
impl SimpleState for LevelEditorState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        init_camera(world, (ARENA_WIDTH as f32, ARENA_HEIGHT as f32));

        self.sprite_sheet = Some(load_sprite_sheet(world, "colored_tilemap_packed"));
        self.entities = if let Some(s) = self.sprite_sheet.clone() {
            load_level(world, s, &self.data)
        } else {
            Vec::new()
        };

        let high = {
            let spr = SpriteRender::new(load_sprite_sheet(world, "highlighter"), 0);
            let tt = TileTransform::new(0, 0);

            let mut trans = Transform::default();
            trans.set_translation_z(0.8);

            world
                .create_entity()
                .with(spr)
                .with(tt)
                .with(trans)
                .with(Animator::default())
                .build()
        };
        self.highlighter = Some(high);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;
        if let StateEvent::Input(e) = event {
            let mut needs_to_update = None;
            let world = data.world;

            match e {
                InputEvent::KeyReleased { key_code, .. } => {
                    use VirtualKeyCode::*;
                    let mut working_version = self.current_editing;
                    let (x, y) = (self.current_editing.x, self.current_editing.y);
                    match key_code {
                        A | Left => working_version.x -= 1,
                        D | Right => working_version.x += 1,
                        W | Up => working_version.y -= 1,
                        S | Down => working_version.y += 1,
                        E | I => self.saved_state = self.data[x as usize][y as usize],
                        Return | Space => {
                            self.data[x as usize][y as usize] = self.saved_state;
                            needs_to_update = Some(self.current_editing);
                        }
                        X => {
                            self.save_self();
                            t = SimpleTrans::Switch(Box::new(
                                LevelEditorLevelSelectState::default(),
                            ));
                        }
                        F12 => {
                            t = SimpleTrans::Switch(
                                Box::new(LevelEditorLevelSelectState::default()),
                            )
                        }
                        _ => {}
                    };

                    let (x_limit, y_limit) = (WIDTH as i32 - 1, HEIGHT as i32 - 1);

                    if working_version.x < 0 {
                        working_version.x = x_limit;
                    } else if working_version.x > x_limit {
                        working_version.x = 0;
                    }
                    #[allow(clippy::if_same_then_else)]
                    if working_version.y < 0 {
                        working_version.y = y_limit;
                    } else if working_version.y > y_limit {
                        working_version.y = y_limit;
                    }

                    let old = self.current_editing;
                    self.current_editing = working_version;

                    let mut tiletransforms = world.write_component::<TileTransform>();
                    let mut animators = world.write_component::<Animator>();
                    if let Some(high) = self.highlighter {
                        if let Some(an) = animators.get_mut(high) {
                            let data = AnimationData::new_no_rotate(
                                old,
                                working_version,
                                PLAYER_MOVEMENT_ANIM_LEN,
                                AnimInterpolation::Linear,
                            );
                            an.replace_data(data);
                        }
                        if let Some(tt) = tiletransforms.get_mut(high) {
                            tt.set(working_version);
                        }
                    }
                }
                InputEvent::MouseWheelMoved(dir) => {
                    let (x, y) = (
                        self.current_editing.x as usize,
                        self.current_editing.y as usize,
                    );

                    //region change self.data
                    let current_state: SpriteRequest = self.data[x][y];

                    let mut index = LIST_OF_ALL_SPRITEREQUESTS
                        .iter()
                        .position(|e| e == &current_state)
                        .unwrap_or(0) as i32;
                    let limit = LIST_OF_ALL_SPRITEREQUESTS.len() as i32 - 1;

                    #[allow(clippy::collapsible_else_if)]
                    if index == 9999 {
                        if dir == ScrollDirection::ScrollUp {
                            index = 0;
                        } else {
                            index = limit;
                        }
                    } else {
                        if dir == ScrollDirection::ScrollUp {
                            index += 1;
                        } else {
                            index -= 1;
                        }
                    }

                    if index < 0 {
                        index = limit;
                    } else if index > limit {
                        index = 0;
                    }

                    let new_state: SpriteRequest = *LIST_OF_ALL_SPRITEREQUESTS
                        .get(index as usize)
                        .unwrap_or(&SpriteRequest::Blank);
                    log::info!("From {:?} to {:?}.", current_state, new_state);
                    self.data[x][y] = new_state;

                    needs_to_update = Some(TileTransform::new(x as i32, y as i32));
                }
                _ => {}
            }

            if let Some(tt_editing) = needs_to_update {
                let (x, y) = (tt_editing.x as usize, tt_editing.y as usize);
                let new_state = self.data[x][y];
                if let Some(ent) = self.entities.get_mut(x) {
                    if let Some(ent) = ent.get_mut(y) {
                        let mut sprite_renderers = world.write_component::<SpriteRender>();
                        let spr = sprite_renderers.get_mut(*ent);
                        let trying_ind = new_state.get_spritesheet_index();

                        #[allow(clippy::collapsible_else_if)]
                        if trying_ind == 9999 {
                            sprite_renderers.remove(*ent);
                        } else {
                            if let Some(spr) = spr {
                                spr.sprite_number = trying_ind;
                            } else {
                                if let Some(sheet) = self.sprite_sheet.clone() {
                                    sprite_renderers
                                        .insert(*ent, SpriteRender::new(sheet, trying_ind))
                                        .unwrap_or_else(|err| {
                                            log::warn!(
                                                "Error Inserting SpriteRenderer into entity: {}",
                                                err
                                            );
                                            None
                                        });
                                }
                            }
                        }
                    }
                }
            }
        }

        t
    }
}

fn load_level(
    world: &mut World,
    sprites_handle: Handle<SpriteSheet>,
    lvl: &Room,
) -> Vec<Vec<Entity>> {
    let mut list = Vec::new();

    for x in 0..WIDTH {
        let mut inside_list = Vec::new();
        for y in 0..HEIGHT {
            let mut none_found = true;

            let tt = TileTransform::new(x as i32, y as i32);
            let mut trans = Transform::default();
            trans.set_translation_z(0.5);

            if let Some(spr_index) = lvl.get(x as usize) {
                if let Some(spr_index) = spr_index.get(y as usize) {
                    let spr_index = spr_index.get_spritesheet_index();
                    if spr_index == 9999 {
                        none_found = true;
                    } else {
                        none_found = false;
                        let spr = SpriteRender::new(sprites_handle.clone(), spr_index);
                        inside_list.push(
                            world
                                .create_entity()
                                .with(spr)
                                .with(tt)
                                .with(trans.clone())
                                .build(),
                        );
                    }
                }
            }

            if none_found {
                inside_list.push(world.create_entity().with(tt).with(trans).build());
            }
        }
        list.push(inside_list);
    }

    list
}
