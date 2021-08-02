use crate::level::Room;
use amethyst::{SimpleState, StateData, GameData};
use crate::states::game_state::LEVELS;
use amethyst::core::ecs::WorldExt;

pub struct LevelEditorState {
	data: Room
}

impl LevelEditorState {
	pub fn new (index: &Option<usize>) -> Self {
		let mut data = Room::default();
		if let Some(index) = index {
			if let Some(lvl) = LEVELS.get(*index) {
				data = Room::new(lvl.as_str());
			}
		}
		Self {
			data
		}
	}
}
impl SimpleState for LevelEditorState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		world.delete_all();
		log::info!("{:?}", self.data);
	}
}