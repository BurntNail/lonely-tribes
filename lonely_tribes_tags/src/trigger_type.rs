#[derive(Copy, Clone, Debug)]
pub enum TriggerType {
	Door,
	Player(usize),
}
impl TriggerType {
	#[allow(dead_code)]
	pub fn get_id(&self) -> usize {
		match self {
			Self::Door => 10,
			Self::Player(u) => *u,
		}
	}
	pub fn from_id(id: &usize) -> Self {
		use TriggerType::*;
		match id {
			10 => Door,
			0 => Player(0),
			1 => Player(1),
			2 => Player(2),
			_ => Player(3),
		}
	}
}