use std::collections::HashMap;

use crate::puzzle::direction::Direction;

use super::{keypad::Keypad, keypad_state::KeypadState, keypad_value::KeypadValue};

pub struct KeypadSimple {
    transitions: HashMap<KeypadState, KeypadValue>,
}

impl KeypadSimple {
    pub fn new() -> Self {
        let mut transitions = HashMap::new();

        // 1 2 3
        // 4 5 6
        // 7 8 9
        Self::add_transition('1', '1', '2', '1', '4', &mut transitions);
        Self::add_transition('2', '1', '3', '2', '5', &mut transitions);
        Self::add_transition('3', '2', '3', '3', '6', &mut transitions);
        Self::add_transition('4', '4', '5', '1', '7', &mut transitions);
        Self::add_transition('5', '4', '6', '2', '8', &mut transitions);
        Self::add_transition('6', '5', '6', '3', '9', &mut transitions);
        Self::add_transition('7', '7', '8', '4', '7', &mut transitions);
        Self::add_transition('8', '7', '9', '5', '8', &mut transitions);
        Self::add_transition('9', '8', '9', '6', '9', &mut transitions);

        Self { transitions }
    }

    fn add_transition(
        value: KeypadValue,
        left: KeypadValue,
        right: KeypadValue,
        up: KeypadValue,
        down: KeypadValue,
        transitions: &mut HashMap<KeypadState, KeypadValue>,
    ) {
        transitions.insert(KeypadState::new(value, Direction::Left), left);
        transitions.insert(KeypadState::new(value, Direction::Right), right);
        transitions.insert(KeypadState::new(value, Direction::Up), up);
        transitions.insert(KeypadState::new(value, Direction::Down), down);
    }
}

impl Default for KeypadSimple {
    fn default() -> Self {
        Self::new()
    }
}

impl Keypad for KeypadSimple {
    fn transition(&self, value: KeypadValue, direction: &Direction) -> KeypadValue {
        *self
            .transitions
            .get(&KeypadState {
                value,
                direction: direction.clone(),
            })
            .unwrap_or_else(|| panic!("Unsupported transition '{}':'{:?}'", value, direction))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::instructions::Instructions;

    use super::*;

    #[test]
    fn test_transitions_1() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('1', &Direction::Left), '1');
        assert_eq!(keypad.transition('1', &Direction::Right), '2');
        assert_eq!(keypad.transition('1', &Direction::Up), '1');
        assert_eq!(keypad.transition('1', &Direction::Down), '4');
    }

    #[test]
    fn test_transitions_2() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('2', &Direction::Left), '1');
        assert_eq!(keypad.transition('2', &Direction::Right), '3');
        assert_eq!(keypad.transition('2', &Direction::Up), '2');
        assert_eq!(keypad.transition('2', &Direction::Down), '5');
    }

    #[test]
    fn test_transitions_3() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('3', &Direction::Left), '2');
        assert_eq!(keypad.transition('3', &Direction::Right), '3');
        assert_eq!(keypad.transition('3', &Direction::Up), '3');
        assert_eq!(keypad.transition('3', &Direction::Down), '6');
    }

    #[test]
    fn test_transitions_4() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('4', &Direction::Left), '4');
        assert_eq!(keypad.transition('4', &Direction::Right), '5');
        assert_eq!(keypad.transition('4', &Direction::Up), '1');
        assert_eq!(keypad.transition('4', &Direction::Down), '7');
    }

    #[test]
    fn test_transitions_5() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('5', &Direction::Left), '4');
        assert_eq!(keypad.transition('5', &Direction::Right), '6');
        assert_eq!(keypad.transition('5', &Direction::Up), '2');
        assert_eq!(keypad.transition('5', &Direction::Down), '8');
    }

    #[test]
    fn test_transitions_6() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('6', &Direction::Left), '5');
        assert_eq!(keypad.transition('6', &Direction::Right), '6');
        assert_eq!(keypad.transition('6', &Direction::Up), '3');
        assert_eq!(keypad.transition('6', &Direction::Down), '9');
    }

    #[test]
    fn test_transitions_7() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('7', &Direction::Left), '7');
        assert_eq!(keypad.transition('7', &Direction::Right), '8');
        assert_eq!(keypad.transition('7', &Direction::Up), '4');
        assert_eq!(keypad.transition('7', &Direction::Down), '7');
    }

    #[test]
    fn test_transitions_8() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('8', &Direction::Left), '7');
        assert_eq!(keypad.transition('8', &Direction::Right), '9');
        assert_eq!(keypad.transition('8', &Direction::Up), '5');
        assert_eq!(keypad.transition('8', &Direction::Down), '8');
    }

    #[test]
    fn test_transitions_9() {
        let keypad = KeypadSimple::new();

        assert_eq!(keypad.transition('9', &Direction::Left), '8');
        assert_eq!(keypad.transition('9', &Direction::Right), '9');
        assert_eq!(keypad.transition('9', &Direction::Up), '6');
        assert_eq!(keypad.transition('9', &Direction::Down), '9');
    }

    #[test]
    fn test_calculate_code() {
        let mut instructions = Instructions::new();

        instructions.push(vec![Direction::Up, Direction::Left, Direction::Left]);
        instructions.push(vec![
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Down,
            Direction::Down,
        ]);
        instructions.push(vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]);
        instructions.push(vec![
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Down,
        ]);

        let keypad = KeypadSimple::new();

        assert_eq!(keypad.calculate_code(&instructions), "1985");
    }
}
