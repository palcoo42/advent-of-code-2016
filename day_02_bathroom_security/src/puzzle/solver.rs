use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{instructions::Instructions, keypad::Keypad, parser::Parser};

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

    fn parse_input_file(&mut self, lines: Vec<String>) -> PuzzleResult {
        self.instructions = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let mut keypad = Keypad::new();
        let code = keypad.calculate_code(&self.instructions);

        Ok(code)
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }
}
