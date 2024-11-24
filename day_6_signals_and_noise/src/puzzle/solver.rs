use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{message::Message, parser::Parser};

pub struct Solver {
    message: Message,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            message: Message::new(vec![]),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 6: Signals and Noise ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.message = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        Ok(self.message.find_message_max_occurrences())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(self.message.find_message_min_occurrences())
    }
}
