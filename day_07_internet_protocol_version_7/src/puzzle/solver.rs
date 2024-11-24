use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{ip_v7::Ipv7, parser::Parser};

pub struct Solver {
    addresses: Vec<Ipv7>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            addresses: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 07: Internet Protocol Version 7 ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.addresses = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let count = self
            .addresses
            .iter()
            .filter(|&ipv7| ipv7.is_tls_supported())
            .count();

        Ok(count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }
}
