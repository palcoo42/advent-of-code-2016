use std::collections::HashSet;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::grids::direction::Direction;
use puzzler::grids::point::Point;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::instruction::Instruction;

pub struct Solution {
    instructions: Vec<Instruction>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 1: No Time for a Taxicab ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_01.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_01.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if lines.len() != 1 {
            return Err(format!("Exactly one line is expected, found {}", lines.len()).into());
        };

        let lines_instr = lines[0]
            .split_terminator(",")
            .map(|s| String::from(s.trim()))
            .collect::<Vec<_>>();

        self.instructions = Parser::parse_lines_with_regex(lines_instr, r"^(\w)(\d+)", |params| {
            if params.len() != 2 {
                return Err(
                    format!("Invalid number of params '{}', expected 2", params.len()).into(),
                );
            }

            let steps = params[1]
                .parse::<usize>()
                .map_err(|_err| format!("Failed to convert '{}' to usize", params[1]))?;

            let instr = match params[0].as_str() {
                "L" => Instruction::Left { steps },
                "R" => Instruction::Right { steps },
                _ => return Err(format!("Invalid instruction '{}'", params[0]).into()),
            };

            Ok(instr)
        })?;

        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let count = Solution::count_blocks(&self.instructions);
        Ok(count.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let count = Solution::count_first_location_to_visit_twice(&self.instructions);
        Ok(count.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn count_blocks(instructions: &[Instruction]) -> usize {
        // Initialize
        let mut point = Point::new(0, 0);
        let mut dir = Direction::North;

        for inst in instructions {
            // Get new direction
            let (new_dir, steps) = match inst {
                Instruction::Left { steps } => (dir.left(), steps),
                Instruction::Right { steps } => (dir.right(), steps),
            };

            // Calculate new point
            point = point.neighbor_at(&new_dir, *steps as isize);
            dir = new_dir;
        }

        (point.x.abs() + point.y.abs()) as usize
    }

    fn count_first_location_to_visit_twice(instructions: &[Instruction]) -> usize {
        let mut locations = HashSet::new();

        // Initialize
        let mut point = Point::new(0, 0);
        let mut dir = Direction::North;

        locations.insert(point);

        for inst in instructions {
            // Get new direction
            let (new_dir, steps) = match inst {
                Instruction::Left { steps } => (dir.left(), steps),
                Instruction::Right { steps } => (dir.right(), steps),
            };

            // Calculate new points
            let new_points = point.neighbor_at_path(&new_dir, *steps as isize);

            dir = new_dir;
            point = *new_points.last().expect("Last element not found");

            for point in new_points {
                if !locations.insert(point) {
                    return (point.x.abs() + point.y.abs()) as usize;
                }
            }
        }

        usize::MAX
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{instruction::Instruction, solution::Solution};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "278");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "161");
    }

    #[test]
    fn test_count_blocks() {
        assert_eq!(
            Solution::count_blocks(&[
                Instruction::Right { steps: 2 },
                Instruction::Left { steps: 3 }
            ]),
            5
        );
        assert_eq!(
            Solution::count_blocks(&[
                Instruction::Right { steps: 2 },
                Instruction::Right { steps: 2 },
                Instruction::Right { steps: 2 },
            ]),
            2
        );
        assert_eq!(
            Solution::count_blocks(&[
                Instruction::Right { steps: 5 },
                Instruction::Left { steps: 5 },
                Instruction::Right { steps: 5 },
                Instruction::Right { steps: 3 }
            ]),
            12
        );
    }

    #[test]
    fn test_count_first_lcoation_to_visit_twice() {
        // R8, R4, R4, R8
        assert_eq!(
            Solution::count_first_location_to_visit_twice(&[
                Instruction::Right { steps: 8 },
                Instruction::Right { steps: 4 },
                Instruction::Right { steps: 4 },
                Instruction::Right { steps: 8 }
            ]),
            4
        );
    }
}
