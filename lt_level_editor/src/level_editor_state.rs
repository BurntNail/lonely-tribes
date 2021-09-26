use crate::{screen_into_tiletransform, sprite_picker::SpritePickerState};
use amethyst::{
    assets::Handle,
    core::{
        ecs::{Builder, Entity, World, WorldExt},
        Transform,
    },
    input::{InputEvent, InputHandler, StringBindings, VirtualKeyCode},
    renderer::{SpriteRender, SpriteSheet},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};
use lonely_tribes_generation::level::{Room, SpriteRequest, INDEX_TO_SPRITEREQUEST, REVERSED_SPRITESHEET_SWATCH_HASHMAP};
use lonely_tribes_lib::{
    states_util::{get_levels_str, init_camera, load_sprite_sheet, CAMERA_DIMENSIONS},
    HEIGHT, WIDTH,
};
use lonely_tribes_tiletransform::{tile_transform::TileTransform, works::tile_works};
use amethyst::winit::MouseButton;
use image::{ImageBuffer, Rgba};
use lonely_tribes_lib::paths::get_directory;

pub(crate) type StateType =
    dyn amethyst::State<amethyst::GameData<'static, 'static>, amethyst::StateEvent> + 'static;

pub const LEVEL_TO_LOAD: usize = 5;

///State for in a level editor
#[derive(Default)]
pub struct LevelEditorState {
    to_go_to_afterwards: Option<Box<StateType>>,
    end_path: String,
    level_loaded: usize,
    
    spritesheet: Option<Handle<SpriteSheet>>,
    
    buttons: Vec<Vec<Entity>>,
    currently_editing: TileTransform,
    previously_editing: TileTransform,
    
    highlighter: Option<Entity>,
}

#[derive(Default)]
pub struct CurrentEditingSprite(pub SpriteRequest);

impl LevelEditorState {
    pub fn new(to_go_to_afterwards: Box<StateType>, level_to_load: usize, end_path: String) -> Self {
        Self {
            to_go_to_afterwards: Some(to_go_to_afterwards),
            end_path,
            level_loaded: level_to_load,
            ..Default::default()
        }
    }
}

impl SimpleState for LevelEditorState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        world.insert(CurrentEditingSprite::default());

        init_camera(world, *CAMERA_DIMENSIONS);

        self.spritesheet = Some(load_sprite_sheet(world, "colored_tilemap_packed"));
        let room_to_load = get_levels_str()[self.level_loaded].clone();
        self.buttons = load_level(
            world,
            self.spritesheet.clone().unwrap(),
            Room::new(room_to_load),
        );

        self.highlighter = Some(get_highlighter(world, self.currently_editing));
    }
    
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;
        let world = data.world;
        
        if let StateEvent::Input(event) = event {
            match event {
                InputEvent::KeyReleased { key_code, .. } => {
                    use VirtualKeyCode::*;
                    match key_code {
                        S => {
                            let mut to_unhide = Vec::new();
                            for l in &self.buttons {
                                for e in l {
                                    to_unhide.push(*e);
                                }
                            }
                            if let Some(e) = &self.highlighter {
                                to_unhide.push(*e);
                            }
                    
                            t = SimpleTrans::Push(Box::new(SpritePickerState::new(to_unhide)));
                        },
                        Return => {
                            self.set_current_sprite_at_current_pos(world);
                        },
                        Escape => {
                            t = SimpleTrans::Switch(self.to_go_to_afterwards.take().expect("Unable to find level select screen"));
                            //TODO: think about coverting to pop
                        },
                        W => {
                            self.save(world);
                            t = SimpleTrans::Switch(self.to_go_to_afterwards.take().expect("Unable to find level select screen"));
                        }
                        _ => {}
                    }
                }
                InputEvent::MouseButtonPressed(btn) => {
                    if btn == MouseButton::Left {
                        self.set_current_sprite_at_current_pos(world);
                    }
                }
                _ => {}
            }
        }

        t
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = data.world;
        
        {
            let input = world.read_resource::<InputHandler<StringBindings>>();

            if let Some((x, y)) = input.mouse_position() {
                let done = screen_into_tiletransform(x, y, world);
                if tile_works(done, &[], WIDTH as i32, HEIGHT as i32) {
                    self.currently_editing = done;
                }
            }
        }

        {
            if self.previously_editing != self.currently_editing {
                self.previously_editing = self.currently_editing;

                if let Some(ent) = self.highlighter {
                    let mut tiles = world.write_storage::<TileTransform>();
                    tiles
                        .insert(ent, self.currently_editing)
                        .unwrap_or_else(|err| {
                            log::warn!("Error updating posiiton of highlighter: {}", err);
                            None
                        });
                }
            }
        }
        SimpleTrans::None
    }
}

impl LevelEditorState {
    fn set_current_sprite_at_current_pos (&self, world: &World) {
        let ent = self.buttons[self.currently_editing.x as usize][self.currently_editing.y as usize];
        let index = {
            world.read_resource::<CurrentEditingSprite>().0.get_spritesheet_index()
        };
    
        let mut spriterenders = world.write_storage::<SpriteRender>();
        spriterenders.insert(ent, SpriteRender::new(self.spritesheet.clone().unwrap(), index))
            .unwrap_or_else(|err| {
                log::warn!("Unable to change level editor sprite: {}", err);
                None
            });
    }

    fn save (&self, world: &World) {
        let spritemap: Vec<Vec<SpriteRequest>> = {
            let spriterenders = world.read_storage::<SpriteRender>();

            self.buttons.clone().into_iter()
                .map(|list| {
                    list.into_iter().map(|entity| {
                        let mut res = SpriteRequest::Blank;
                        if let Some(spr) = spriterenders.get(entity) {
                            if let Some(s) = INDEX_TO_SPRITEREQUEST.get(&spr.sprite_number) {
                                res = *s;
                            }
                        }

                        res
                    }).collect()
                }).collect()
        };

        let mut img = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        for (x, y, px) in img.enumerate_pixels_mut() {
             *px = *REVERSED_SPRITESHEET_SWATCH_HASHMAP
                .get(&spritemap[x as usize][y as usize])
                .unwrap_or(&Rgba::from([0; 4]));
        }

        // let index = get_levels().into_iter().filter(|(_, t)| t == &LevelType::User).count();

        let path = get_directory(false).join("../maps/").join(&self.end_path);
        img.save(path)
            .unwrap_or_else(|err| log::error!("Error saving level editor level: {}", err));
    }
}

fn get_highlighter(world: &mut World, default_pos: TileTransform) -> Entity {
    let spr = SpriteRender::new(load_sprite_sheet(world, "highlighter"), 0);
    world
        .create_entity()
        .with(Transform::default())
        .with(default_pos)
        .with(spr)
        .build()
}

fn load_level(
    world: &mut World,
    sprites_handle: Handle<SpriteSheet>,
    lvl: Room,
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
                    if spr_index != 9999 {
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
