use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{city::City, document::Document, parser::PuzzleParser};

pub struct Solver {
    document: Document,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            document: Document::new(vec![]),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 1: No Time for a Taxicab ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.document = PuzzleParser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let mut city = City::new();
        let blocks = city.count_blocks_to_hq(&self.document);

        Ok(blocks.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let mut city = City::new();
        let blocks = city.find_first_intersection(&self.document);

        Ok(blocks.to_string())
    }
}
