use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::triangle::Triangle;

pub struct Solution {
    triangles: Vec<Triangle>,
    column_triangles: Vec<Triangle>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 3: Squares With Three Sides ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_03.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_03.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        let numbers = Parser::parse_lines_to_unsigned_integers(lines)?;
        self.triangles = numbers
            .iter()
            .map(|nrs| Triangle::new(nrs[0], nrs[1], nrs[2]))
            .collect();
        self.column_triangles = Self::column_triangles(&numbers);
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let count = self
            .triangles
            .iter()
            .filter(|triangle| triangle.is_valid())
            .count();
        Ok(count.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let count = self
            .column_triangles
            .iter()
            .filter(|triangle| triangle.is_valid())
            .count();
        Ok(count.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            column_triangles: Vec::new(),
        }
    }

    fn column_triangles(lines: &[Vec<usize>]) -> Vec<Triangle> {
        if lines.len() % 3 != 0 {
            panic!("Raw lines shall be multiple of 3");
        }

        // Put numbers into a vector of consecutive values
        let mut numbers = Vec::with_capacity(lines.len() * lines[0].len());

        for col in 0..lines[0].len() {
            for line in lines {
                numbers.push(line[col]);
            }
        }

        // Create rectangles from 3 consecutive elements
        numbers
            .chunks(3)
            .map(|chunk| Triangle::new(chunk[0], chunk[1], chunk[2]))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{solution::Solution, triangle::Triangle};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "917");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "1649");
    }

    #[test]
    fn test_column_triangles() {
        let triangles = vec![
            vec![101, 301, 501],
            vec![102, 302, 502],
            vec![103, 303, 503],
            vec![201, 401, 601],
            vec![202, 402, 602],
            vec![203, 403, 603],
        ];

        let column_triangles = vec![
            Triangle::new(101, 102, 103),
            Triangle::new(201, 202, 203),
            Triangle::new(301, 302, 303),
            Triangle::new(401, 402, 403),
            Triangle::new(501, 502, 503),
            Triangle::new(601, 602, 603),
        ];

        assert_eq!(Solution::column_triangles(&triangles), column_triangles);
    }
}
