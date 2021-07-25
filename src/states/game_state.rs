use crate::components::{Collider, ColliderList, TileTransform};
use crate::components::{GameWinState, WinStateEnum, NPC};
use crate::level::Room;
use crate::states::states_util::{get_trans, init_camera, load_sprite_sheet};
use crate::states::PostGameState;
use crate::systems::UpdateTileTransforms;
use crate::tag::Tag;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::assets::{Handle, Loader};
use amethyst::input::{InputEvent, VirtualKeyCode};
use amethyst::renderer::SpriteRender;
use amethyst::ui::{FontAsset, TtfFormat};
use amethyst::{
    assets::AssetStorage,
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct PuzzleState {
    handle: Option<Handle<SpriteSheet>>,
    ws: WinStateEnum,
}
impl Default for PuzzleState {
    fn default() -> Self {
        Self {
            handle: None,
            ws: WinStateEnum::default(),
        }
    }
}

impl SimpleState for PuzzleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        init_camera(world, (ARENA_WIDTH as f32, ARENA_HEIGHT as f32));

        self.handle
            .replace(load_sprite_sheet(world, "art/colored_tilemap_packed"));

        world.register::<crate::components::NPC>();
        world.insert(GameWinState::default());

        let lvl_path = "assets/maps/test-room-one.png"; //TODO: Fix FQDN
        load_level(world, self.handle.clone().unwrap(), lvl_path);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();
        log::info!("Deleted all entities");

        if let WinStateEnum::End { won } = self.ws {
            world.insert(GameWinState::new(Some(won)))
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        get_trans(event)
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let game_state = data.world.read_resource::<GameWinState>();
        let ws = game_state.ws;
        self.ws = ws;

        match ws {
            WinStateEnum::End { .. } => Trans::Switch(Box::new(PostGameState)),
            WinStateEnum::TBD => Trans::None,
        }
    }
}

fn load_level(world: &mut World, sprites_handle: Handle<SpriteSheet>, path: &str) {
    let lvl = Room::new(path);

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
                        .with(Collider::new(true, id))
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
                        .with(Collider::new(true, trigger_type.get_id()))
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
