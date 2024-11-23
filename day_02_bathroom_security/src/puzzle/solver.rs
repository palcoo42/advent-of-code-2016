use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{
    instructions::Instructions,
    keypads::{keypad::Keypad, keypad_complex::KeypadComplex, keypad_simple::KeypadSimple},
    parser::Parser,
};

pub struct Solver {
    instructions: Instructions,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            instructions: Instructions::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 02: Bathroom Security ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.instructions = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let keypad = KeypadSimple::new();
        let code = keypad.calculate_code(&self.instructions);

        Ok(code)
    }

    fn part_2(&self) -> SolutionResult {
        let keypad = KeypadComplex::new();
        let code = keypad.calculate_code(&self.instructions);

        Ok(code)
    }
}
