use std::collections::HashMap;

use crate::puzzle::direction::Direction;

use super::{keypad::Keypad, keypad_state::KeypadState, keypad_value::KeypadValue};

pub struct KeypadComplex {
    transitions: HashMap<KeypadState, KeypadValue>,
}

impl KeypadComplex {
    pub fn new() -> Self {
        let mut transitions = HashMap::new();

        //       1
        //     2 3 4
        //   5 6 7 8 9
        //     A B C
        //       D
        Self::add_transition('1', '1', '1', '1', '3', &mut transitions);
        Self::add_transition('2', '2', '3', '2', '6', &mut transitions);
        Self::add_transition('3', '2', '4', '1', '7', &mut transitions);
        Self::add_transition('4', '3', '4', '4', '8', &mut transitions);
        Self::add_transition('5', '5', '6', '5', '5', &mut transitions);
        Self::add_transition('6', '5', '7', '2', 'A', &mut transitions);
        Self::add_transition('7', '6', '8', '3', 'B', &mut transitions);
        Self::add_transition('8', '7', '9', '4', 'C', &mut transitions);
        Self::add_transition('9', '8', '9', '9', '9', &mut transitions);
        Self::add_transition('A', 'A', 'B', '6', 'A', &mut transitions);
        Self::add_transition('B', 'A', 'C', '7', 'D', &mut transitions);
        Self::add_transition('C', 'B', 'C', '8', 'C', &mut transitions);
        Self::add_transition('D', 'D', 'D', 'B', 'D', &mut transitions);

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

impl Default for KeypadComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl Keypad for KeypadComplex {
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
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('1', &Direction::Left), '1');
        assert_eq!(keypad.transition('1', &Direction::Right), '1');
        assert_eq!(keypad.transition('1', &Direction::Up), '1');
        assert_eq!(keypad.transition('1', &Direction::Down), '3');
    }

    #[test]
    fn test_transitions_2() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('2', &Direction::Left), '2');
        assert_eq!(keypad.transition('2', &Direction::Right), '3');
        assert_eq!(keypad.transition('2', &Direction::Up), '2');
        assert_eq!(keypad.transition('2', &Direction::Down), '6');
    }

    #[test]
    fn test_transitions_3() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('3', &Direction::Left), '2');
        assert_eq!(keypad.transition('3', &Direction::Right), '4');
        assert_eq!(keypad.transition('3', &Direction::Up), '1');
        assert_eq!(keypad.transition('3', &Direction::Down), '7');
    }

    #[test]
    fn test_transitions_4() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('4', &Direction::Left), '3');
        assert_eq!(keypad.transition('4', &Direction::Right), '4');
        assert_eq!(keypad.transition('4', &Direction::Up), '4');
        assert_eq!(keypad.transition('4', &Direction::Down), '8');
    }

    #[test]
    fn test_transitions_5() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('5', &Direction::Left), '5');
        assert_eq!(keypad.transition('5', &Direction::Right), '6');
        assert_eq!(keypad.transition('5', &Direction::Up), '5');
        assert_eq!(keypad.transition('5', &Direction::Down), '5');
    }

    #[test]
    fn test_transitions_6() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('6', &Direction::Left), '5');
        assert_eq!(keypad.transition('6', &Direction::Right), '7');
        assert_eq!(keypad.transition('6', &Direction::Up), '2');
        assert_eq!(keypad.transition('6', &Direction::Down), 'A');
    }

    #[test]
    fn test_transitions_7() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('7', &Direction::Left), '6');
        assert_eq!(keypad.transition('7', &Direction::Right), '8');
        assert_eq!(keypad.transition('7', &Direction::Up), '3');
        assert_eq!(keypad.transition('7', &Direction::Down), 'B');
    }

    #[test]
    fn test_transitions_8() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('8', &Direction::Left), '7');
        assert_eq!(keypad.transition('8', &Direction::Right), '9');
        assert_eq!(keypad.transition('8', &Direction::Up), '4');
        assert_eq!(keypad.transition('8', &Direction::Down), 'C');
    }

    #[test]
    fn test_transitions_9() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('9', &Direction::Left), '8');
        assert_eq!(keypad.transition('9', &Direction::Right), '9');
        assert_eq!(keypad.transition('9', &Direction::Up), '9');
        assert_eq!(keypad.transition('9', &Direction::Down), '9');
    }

    #[test]
    fn test_transitions_a() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('A', &Direction::Left), 'A');
        assert_eq!(keypad.transition('A', &Direction::Right), 'B');
        assert_eq!(keypad.transition('A', &Direction::Up), '6');
        assert_eq!(keypad.transition('A', &Direction::Down), 'A');
    }

    #[test]
    fn test_transitions_b() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('B', &Direction::Left), 'A');
        assert_eq!(keypad.transition('B', &Direction::Right), 'C');
        assert_eq!(keypad.transition('B', &Direction::Up), '7');
        assert_eq!(keypad.transition('B', &Direction::Down), 'D');
    }

    #[test]
    fn test_transitions_c() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('C', &Direction::Left), 'B');
        assert_eq!(keypad.transition('C', &Direction::Right), 'C');
        assert_eq!(keypad.transition('C', &Direction::Up), '8');
        assert_eq!(keypad.transition('C', &Direction::Down), 'C');
    }

    #[test]
    fn test_transitions_d() {
        let keypad = KeypadComplex::new();

        assert_eq!(keypad.transition('D', &Direction::Left), 'D');
        assert_eq!(keypad.transition('D', &Direction::Right), 'D');
        assert_eq!(keypad.transition('D', &Direction::Up), 'B');
        assert_eq!(keypad.transition('D', &Direction::Down), 'D');
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

        let keypad = KeypadComplex::new();

        assert_eq!(keypad.calculate_code(&instructions), "5DB3");
    }
}
