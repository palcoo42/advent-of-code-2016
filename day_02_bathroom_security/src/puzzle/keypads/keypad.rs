use crate::puzzle::{direction::Direction, instructions::Instructions};

use super::keypad_value::KeypadValue;

pub trait Keypad {
    /// Calculate code
    fn calculate_code(&self, instructions: &Instructions) -> String {
        // We always start at position '5'
        let mut value = '5';

        // Final code
        let mut code = String::new();

        // Go through all instructions
        for directions in instructions.iter() {
            // Go through all directions to find a code for this instruction
            for direction in directions {
                value = self.transition(value, direction);
            }

            // Append to codes
            code.push(value);
        }

        code
    }

    /// Transition from provided value to next value in direction
    fn transition(&self, value: KeypadValue, direction: &Direction) -> KeypadValue;
}
