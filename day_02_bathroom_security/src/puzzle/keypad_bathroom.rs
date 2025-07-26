use crate::puzzle::{direction::Direction, keypad::Keypad, keypad_mapping::KeypadMapping};

/*
1 2 3
4 5 6
7 8 9
*/
pub struct KeypadBathroom {
    mapping: KeypadMapping<usize>,
}

impl KeypadBathroom {
    fn create_mapping() -> KeypadMapping<usize> {
        let mut mapping = KeypadMapping::new();

        mapping.add(
            1,
            &[
                (Direction::Left, 1),
                (Direction::Right, 2),
                (Direction::Up, 1),
                (Direction::Down, 4),
            ],
        );

        mapping.add(
            2,
            &[
                (Direction::Left, 1),
                (Direction::Right, 3),
                (Direction::Up, 2),
                (Direction::Down, 5),
            ],
        );

        mapping.add(
            3,
            &[
                (Direction::Left, 2),
                (Direction::Right, 3),
                (Direction::Up, 3),
                (Direction::Down, 6),
            ],
        );

        mapping.add(
            4,
            &[
                (Direction::Left, 4),
                (Direction::Right, 5),
                (Direction::Up, 1),
                (Direction::Down, 7),
            ],
        );

        mapping.add(
            5,
            &[
                (Direction::Left, 4),
                (Direction::Right, 6),
                (Direction::Up, 2),
                (Direction::Down, 8),
            ],
        );

        mapping.add(
            6,
            &[
                (Direction::Left, 5),
                (Direction::Right, 6),
                (Direction::Up, 3),
                (Direction::Down, 9),
            ],
        );

        mapping.add(
            7,
            &[
                (Direction::Left, 7),
                (Direction::Right, 8),
                (Direction::Up, 4),
                (Direction::Down, 7),
            ],
        );

        mapping.add(
            8,
            &[
                (Direction::Left, 7),
                (Direction::Right, 9),
                (Direction::Up, 5),
                (Direction::Down, 8),
            ],
        );

        mapping.add(
            9,
            &[
                (Direction::Left, 8),
                (Direction::Right, 9),
                (Direction::Up, 6),
                (Direction::Down, 9),
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

impl Keypad for KeypadBathroom {
    fn get_code(&self, directions: &[Vec<Direction>]) -> String {
        self.mapping.get_code(5, directions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code() {
        let keypad = KeypadBathroom::new();

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
            String::from("1985")
        );
    }
}
