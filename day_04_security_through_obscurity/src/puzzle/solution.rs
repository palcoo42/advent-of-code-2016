use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::room::Room;

pub struct Solution {
    rooms: Vec<Room>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 4: Security Through Obscurity ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_04.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_04.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.rooms = Parser::parse_lines_with_regex(lines, r"(.*)-(\d+)\[(\w+)]$", |parts| {
            if parts.len() != 3 {
                return Err(format!("Exactly 3 parts are expected, found {}", parts.len()).into());
            }

            let name = parts[0].as_str();
            let id = parts[1]
                .parse::<usize>()
                .map_err(|_err| -> Box<dyn Error> {
                    format!("Failed to convert '{}' to usize", parts[1]).into()
                })?;
            let checksum = parts[2].as_str();

            Ok(Room::new(name, id, checksum))
        })?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let real_room_sum = self
            .rooms
            .iter()
            .filter(|room| room.is_real())
            .map(|room| room.get_sector_id())
            .sum::<usize>();

        Ok(real_room_sum.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let storage_name = String::from("northpole object storage");
        let room = self
            .rooms
            .iter()
            .find(|room| room.decode_name() == storage_name)
            .ok_or(format!("Failed to find '{storage_name}'"))?;

        Ok(room.get_sector_id().to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "278221");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "267");
    }
}
