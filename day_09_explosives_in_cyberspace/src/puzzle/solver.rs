use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{compression::Compression, parser::Parser};

pub struct Solver {
    content: String,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 09: Explosives in Cyberspace ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.content = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let decoded = Compression::decompress(&self.content)?;
        Ok(decoded.len().to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let decoded = Compression::decompress_v2(&self.content)?;
        Ok(decoded.to_string())
    }
}
