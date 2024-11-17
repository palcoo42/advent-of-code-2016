use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{city::City, document::Document, puzzle_parser::PuzzleParser};

pub struct Solver {
    document: Document,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            document: Document::new(vec![]),
        }
    }

    fn parse_input_file(&mut self, lines: Vec<String>) -> PuzzleResult {
        self.document = PuzzleParser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let mut city = City::new();
        let blocks = city.count_blocks_to_hq(&self.document);

        Ok(blocks.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }
}
