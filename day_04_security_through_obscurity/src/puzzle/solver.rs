use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
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

    fn parse_input_file(&mut self, lines: &[String]) -> PuzzleResult {
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
        Ok(String::from("not solved"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut solver = Solver::new();
        let lines = [
            String::from("aaaaa-bbb-z-y-x-123[abxyz]"),
            String::from("a-b-c-d-e-f-g-h-987[abcde]"),
            String::from("not-a-real-room-404[oarel]"),
            String::from("totally-real-room-200[decoy]"),
        ];
        assert!(solver.parse_input_file(&lines).is_ok());

        assert_eq!(solver.part_1().unwrap(), "1514");
    }
}
