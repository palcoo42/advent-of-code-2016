use crate::puzzle::direction::Direction;

use super::keypad_value::KeypadValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeypadState {
    pub value: KeypadValue,
    pub direction: Direction,
}

impl KeypadState {
    pub fn new(value: KeypadValue, direction: Direction) -> Self {
        Self { value, direction }
    }
}
