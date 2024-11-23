use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_error::PuzzleError,
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, room::Room};

pub struct Solver {
    rooms: Vec<Room>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self { rooms: Vec::new() }
    }

    fn get_description(&self) -> &str {
        "--- Day 4: Security Through Obscurity ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.rooms = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let sum_valid_rooms = self
            .rooms
            .iter()
            .filter_map(|r| match r.is_real() {
                true => Some(r.get_sector_id()),
                false => None,
            })
            .sum::<u32>();

        Ok(sum_valid_rooms.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        // Look for 'North Pole object'
        let names = self
            .rooms
            .iter()
            .filter_map(|r| {
                if r.decrypt().contains("northpole") {
                    Some(r.get_sector_id())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if names.len() != 1 {
            return Err(PuzzleError::GenericError(format!(
                "Only one solution is expected but '{}' found",
                names.len()
            )));
        }

        Ok(names[0].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut solver = Solver::new();
        let lines = [
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];
        assert!(solver.parse_input_file(&lines).is_ok());

        assert_eq!(solver.part_1().unwrap(), "1514");
    }
}
