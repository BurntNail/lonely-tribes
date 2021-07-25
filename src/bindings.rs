use std::fmt::{Display, Formatter};
use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum AxisBinding {
    Horizontal,
    Vertical
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum ActionBinding {
    Up,
    Down,
    Left,
    Right,
    Restart
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for ActionBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct MovementBindingTypes;

impl BindingTypes for MovementBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}