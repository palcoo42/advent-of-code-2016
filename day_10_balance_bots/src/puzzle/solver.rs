use std::cell::RefCell;

use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{factory::Factory, parser::Parser};

pub struct Solver {
    factory: RefCell<Factory>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            factory: RefCell::new(Factory::default()),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 10: Balance Bots ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.factory = RefCell::new(Factory::new(Parser::parse_lines(lines)?));
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let bot_id = self
            .factory
            .borrow_mut()
            .find_bot_who_compares_chips(&[61, 17])?;

        Ok(bot_id.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let output_bins = self.factory.borrow_mut().calculate_output_bins()?;

        Ok(output_bins.to_string())
    }
}
