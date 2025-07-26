use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::direction::Direction;
use crate::puzzle::keypad::Keypad;
use crate::puzzle::keypad_advanced::KeypadAdvanced;
use crate::puzzle::keypad_bathroom::KeypadBathroom;

pub struct Solution {
    directions: Vec<Vec<Direction>>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 2: Bathroom Security ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_02.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_02.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut instructions = Vec::new();

        for line in lines {
            let directions = line
                .chars()
                .map(|c| match c {
                    'L' => Ok(Direction::Left),
                    'R' => Ok(Direction::Right),
                    'U' => Ok(Direction::Up),
                    'D' => Ok(Direction::Down),
                    d => Err(format!("Unsupported direction '{d}' [{line}]")),
                })
                .collect::<Result<Vec<_>, _>>()?;

            instructions.push(directions);
        }

        self.directions = instructions;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let keyboad = KeypadBathroom::new();
        let code = keyboad.get_code(&self.directions);
        Ok(code.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let keyboad = KeypadAdvanced
        ::new();
        let code = keyboad.get_code(&self.directions);
        Ok(code.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            directions: Vec::new(),
        }
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "73597");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "A47DA");
    }
}
