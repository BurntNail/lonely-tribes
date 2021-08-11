use crate::components::data_holder::EntityHolder;
use amethyst::{
    core::{
        ecs::{Entity, World, WorldExt},
        Hidden,
    },
    input::{InputEvent, VirtualKeyCode},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};

///Resource to optionally disable movement - unless it is true, we assume false as the default is false
pub struct MovementDisabler {
    pub enabled: bool,
}
impl Default for MovementDisabler {
    fn default() -> Self {
        Self { enabled: false }
    }
}

///State for when the game is paused
pub struct PausedState;

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.insert(MovementDisabler { enabled: true });

        let entities = world.read_resource::<EntityHolder>();
        hide_entities(world, entities.get_all_entities());
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;
        if let StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) = event {
            if key_code == VirtualKeyCode::Escape {
                t = SimpleTrans::Pop;

                let world = data.world;
                world.insert(MovementDisabler::default());

                let entities = world.read_resource::<EntityHolder>();
                show_entities(world, entities.get_all_entities())
            }
        }

        t
    }
}

///Function to hide a given list of entities, by adding Hidden components to all of them
pub fn hide_entities(world: &mut World, entities: Vec<Entity>) {
    let mut hiddens = world.write_storage::<Hidden>();

    entities.into_iter().for_each(|ent| {
        hiddens.insert(ent, Hidden).unwrap_or_else(|err| {
            log::warn!("Unable to hide entity: {}", err);
            None
        });
    });
}

///Function to show a given list of entities, by removing Hidden components from all of them
///
///If they don't have a Hidden component, then it does a log::warn
pub fn show_entities(world: &mut World, entities: Vec<Entity>) {
    let mut hiddens = world.write_storage::<Hidden>();

    entities.into_iter().for_each(|ent| {
        hiddens.remove(ent).unwrap_or_else(|| {
            log::warn!("Unable to show entity: {:?}", ent);
            Hidden
        });
    });
}
