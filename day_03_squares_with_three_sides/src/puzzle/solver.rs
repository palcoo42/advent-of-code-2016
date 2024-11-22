use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, triangle::Triangle};

pub struct Solver {
    triangles: Vec<Triangle>,
    triangles_vertical: Vec<Triangle>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            triangles: Vec::new(),
            triangles_vertical: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 02: Bathroom Security ---"
    }

    fn parse_input_file(&mut self, lines: &[String]) -> PuzzleResult {
        self.triangles = Parser::parse_lines(lines)?;
        self.triangles_vertical = Parser::parse_lines_vertically(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let count = self.triangles.iter().filter(|t| t.is_valid()).count();
        Ok(count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let count = self
            .triangles_vertical
            .iter()
            .filter(|t| t.is_valid())
            .count();
        Ok(count.to_string())
    }
}
