use crate::puzzle::{direction::Direction, keypad::Keypad, keypad_mapping::KeypadMapping};

/*
    1
  2 3 4
5 6 7 8 9
  A B C
    D
*/
pub struct KeypadAdvanced {
    mapping: KeypadMapping<char>,
}

impl KeypadAdvanced {
    fn create_mapping() -> KeypadMapping<char> {
        let mut mapping = KeypadMapping::new();

        mapping.add(
            '1',
            &[
                (Direction::Left, '1'),
                (Direction::Right, '1'),
                (Direction::Up, '1'),
                (Direction::Down, '3'),
            ],
        );

        mapping.add(
            '2',
            &[
                (Direction::Left, '2'),
                (Direction::Right, '3'),
                (Direction::Up, '2'),
                (Direction::Down, '6'),
            ],
        );

        mapping.add(
            '3',
            &[
                (Direction::Left, '2'),
                (Direction::Right, '4'),
                (Direction::Up, '1'),
                (Direction::Down, '7'),
            ],
        );

        mapping.add(
            '4',
            &[
                (Direction::Left, '3'),
                (Direction::Right, '4'),
                (Direction::Up, '4'),
                (Direction::Down, '8'),
            ],
        );

        mapping.add(
            '5',
            &[
                (Direction::Left, '5'),
                (Direction::Right, '6'),
                (Direction::Up, '5'),
                (Direction::Down, '5'),
            ],
        );

        mapping.add(
            '6',
            &[
                (Direction::Left, '5'),
                (Direction::Right, '7'),
                (Direction::Up, '2'),
                (Direction::Down, 'A'),
            ],
        );

        mapping.add(
            '7',
            &[
                (Direction::Left, '6'),
                (Direction::Right, '8'),
                (Direction::Up, '3'),
                (Direction::Down, 'B'),
            ],
        );

        mapping.add(
            '8',
            &[
                (Direction::Left, '7'),
                (Direction::Right, '9'),
                (Direction::Up, '4'),
                (Direction::Down, 'C'),
            ],
        );

        mapping.add(
            '9',
            &[
                (Direction::Left, '8'),
                (Direction::Right, '9'),
                (Direction::Up, '9'),
                (Direction::Down, '9'),
            ],
        );

        mapping.add(
            'A',
            &[
                (Direction::Left, 'A'),
                (Direction::Right, 'B'),
                (Direction::Up, '6'),
                (Direction::Down, 'A'),
            ],
        );

        mapping.add(
            'B',
            &[
                (Direction::Left, 'A'),
                (Direction::Right, 'C'),
                (Direction::Up, '7'),
                (Direction::Down, 'D'),
            ],
        );

        mapping.add(
            'C',
            &[
                (Direction::Left, 'B'),
                (Direction::Right, 'C'),
                (Direction::Up, '8'),
                (Direction::Down, 'C'),
            ],
        );

        mapping.add(
            'D',
            &[
                (Direction::Left, 'D'),
                (Direction::Right, 'D'),
                (Direction::Up, 'B'),
                (Direction::Down, 'D'),
            ],
        );

        mapping
    }

    pub fn new() -> Self {
        Self {
            mapping: Self::create_mapping(),
        }
    }
}

impl Keypad for KeypadAdvanced {
    fn get_code(&self, directions: &[Vec<Direction>]) -> String {
        self.mapping.get_code('5', directions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code() {
        let keypad = KeypadAdvanced::new();

        assert_eq!(
            keypad.get_code(&[
                vec![Direction::Up, Direction::Up, Direction::Left],
                vec![
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down
                ],
                vec![
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left
                ],
                vec![
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down
                ]
            ]),
            String::from("5DB3")
        );
    }
}
