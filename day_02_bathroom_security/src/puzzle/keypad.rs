use crate::puzzle::direction::Direction;

pub trait Keypad {
    fn get_code(&self, directions: &[Vec<Direction>]) -> String;
}
