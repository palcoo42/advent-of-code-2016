use super::{direction::Direction, instructions::Instructions, position::Position};

/// Number of rows in keypad
const KEYPAD_ROWS: u32 = 3;

/// Number of columns in keypad
const KEYPAD_COLS: u32 = 3;

#[derive(Debug, Default)]
pub struct Keypad {
    buttons: Vec<Vec<u32>>,
    current: Position,
}

impl Keypad {
    pub fn new() -> Self {
        let mut counter = 0_u32;
        let mut buttons = Vec::new();

        for _ in 0..KEYPAD_ROWS {
            let row_buttons = (0..KEYPAD_COLS)
                .map(|_| {
                    counter += 1;
                    counter
                })
                .collect::<Vec<_>>();

            buttons.push(row_buttons);
        }

        Self {
            buttons,
            current: Position { x: 1, y: 1 },
        }
    }

    pub fn calculate_code(&mut self, instructions: &Instructions) -> String {
        let mut code = String::with_capacity(instructions.len());

        for instruction in instructions.iter() {
            let number = self.execute_instruction(instruction);
            code.push_str(&number.to_string());
        }

        code
    }

    fn execute_instruction(&mut self, directions: &[Direction]) -> u32 {
        for direction in directions {
            self.move_to(direction);
        }

        self.buttons[self.current.x as usize][self.current.y as usize]
    }

    fn move_to(&mut self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => {
                self.current.x = std::cmp::max(0, self.current.x as i32 - 1) as u32;
            }
            Direction::Left => {
                self.current.y = std::cmp::max(0, self.current.y as i32 - 1) as u32;
            }
            Direction::Down => {
                self.current.x = std::cmp::min(self.current.x + 1, KEYPAD_ROWS - 1);
            }
            Direction::Right => {
                self.current.y = std::cmp::min(self.current.y + 1, KEYPAD_COLS - 1);
            }
        }

        self.current.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let keypad = Keypad::new();

        assert_eq!(keypad.buttons[0][0], 1);
        assert_eq!(keypad.buttons[0][1], 2);
        assert_eq!(keypad.buttons[0][2], 3);
        assert_eq!(keypad.buttons[1][0], 4);
        assert_eq!(keypad.buttons[1][1], 5);
        assert_eq!(keypad.buttons[1][2], 6);
        assert_eq!(keypad.buttons[2][0], 7);
        assert_eq!(keypad.buttons[2][1], 8);
        assert_eq!(keypad.buttons[2][2], 9);
    }

    #[test]
    fn test_do_move_x_0_x_0() {
        let mut keypad = Keypad::new();

        keypad.current = Position { x: 0, y: 0 };
        assert_eq!(keypad.move_to(&Direction::Left), Position { x: 0, y: 0 });

        keypad.current = Position { x: 0, y: 0 };
        assert_eq!(keypad.move_to(&Direction::Down), Position { x: 1, y: 0 });

        keypad.current = Position { x: 0, y: 0 };
        assert_eq!(keypad.move_to(&Direction::Right), Position { x: 0, y: 1 });

        keypad.current = Position { x: 0, y: 0 };
        assert_eq!(keypad.move_to(&Direction::Up), Position { x: 0, y: 0 });
    }

    #[test]
    fn test_do_move_x_0_y_max() {
        let mut keypad = Keypad::new();

        let y_max = KEYPAD_COLS - 1;

        keypad.current = Position { x: 0, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Left),
            Position { x: 0, y: y_max - 1 }
        );

        keypad.current = Position { x: 0, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Down),
            Position { x: 1, y: y_max }
        );

        keypad.current = Position { x: 0, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Right),
            Position { x: 0, y: y_max }
        );

        keypad.current = Position { x: 0, y: y_max };
        assert_eq!(keypad.move_to(&Direction::Up), Position { x: 0, y: y_max });
    }

    #[test]
    fn test_do_move_x_max_y_0() {
        let mut keypad = Keypad::new();

        let x_max = KEYPAD_ROWS - 1;

        keypad.current = Position { x: x_max, y: 0 };
        assert_eq!(
            keypad.move_to(&Direction::Left),
            Position { x: x_max, y: 0 }
        );

        keypad.current = Position { x: x_max, y: 0 };
        assert_eq!(
            keypad.move_to(&Direction::Down),
            Position { x: x_max, y: 0 }
        );

        keypad.current = Position { x: x_max, y: 0 };
        assert_eq!(
            keypad.move_to(&Direction::Right),
            Position { x: x_max, y: 1 }
        );

        keypad.current = Position { x: x_max, y: 0 };
        assert_eq!(
            keypad.move_to(&Direction::Up),
            Position { x: x_max - 1, y: 0 }
        );
    }

    #[test]
    fn test_do_move_x_max_y_max() {
        let mut keypad = Keypad::new();

        let x_max = KEYPAD_ROWS - 1;
        let y_max = KEYPAD_COLS - 1;

        keypad.current = Position { x: x_max, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Left),
            Position {
                x: x_max,
                y: y_max - 1
            }
        );

        keypad.current = Position { x: x_max, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Down),
            Position { x: x_max, y: y_max }
        );

        keypad.current = Position { x: x_max, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Right),
            Position { x: x_max, y: y_max }
        );

        keypad.current = Position { x: x_max, y: y_max };
        assert_eq!(
            keypad.move_to(&Direction::Up),
            Position {
                x: x_max - 1,
                y: y_max
            }
        );
    }

    #[test]
    fn test_do_move_x_1_y_1() {
        let mut keypad = Keypad::new();

        keypad.current = Position { x: 1, y: 1 };
        assert_eq!(keypad.move_to(&Direction::Left), Position { x: 1, y: 0 });

        keypad.current = Position { x: 1, y: 1 };
        assert_eq!(keypad.move_to(&Direction::Down), Position { x: 2, y: 1 });

        keypad.current = Position { x: 1, y: 1 };
        assert_eq!(keypad.move_to(&Direction::Right), Position { x: 1, y: 2 });

        keypad.current = Position { x: 1, y: 1 };
        assert_eq!(keypad.move_to(&Direction::Up), Position { x: 0, y: 1 });
    }

    #[test]
    fn test_execute_instruction() {
        let mut keypad = Keypad::new();

        let directions = [Direction::Up, Direction::Left, Direction::Left];
        assert_eq!(keypad.execute_instruction(&directions), 1);

        let directions = [
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Down,
            Direction::Down,
        ];
        assert_eq!(keypad.execute_instruction(&directions), 9);

        let directions = [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        assert_eq!(keypad.execute_instruction(&directions), 8);

        let directions = [
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Down,
        ];
        assert_eq!(keypad.execute_instruction(&directions), 5);
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

        let mut keypad = Keypad::new();

        assert_eq!(keypad.calculate_code(&instructions), "1985");
    }
}
