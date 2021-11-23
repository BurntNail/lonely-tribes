use crate::trigger_type::TriggerType;

#[derive(Copy, Clone, Debug)]
pub enum Tag {
    Player(usize),
    Collision,
    Trigger(TriggerType),
    Other,
}
