use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{
    door::Door,
    passwords::{complex::ComplexPassword, simple::SimplePassword},
};

pub struct Solver {}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {}
    }

    fn get_description(&self) -> &str {
        "--- Day 05: How About a Nice Game of Chess? ---"
    }

    fn parse_input_file(&mut self, _lines: &[&str]) -> PuzzleResult {
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let door = Door::new("reyedfim");
        let pwd = door.decode_password::<SimplePassword>()?;

        Ok(pwd)
    }

    fn part_2(&self) -> SolutionResult {
        let door = Door::new("reyedfim");
        let pwd = door.decode_password::<ComplexPassword>()?;

        Ok(pwd)
    }
}
