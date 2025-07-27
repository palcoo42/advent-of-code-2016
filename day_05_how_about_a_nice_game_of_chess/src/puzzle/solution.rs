use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::hacker::Hacker;

pub struct Solution {}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 5: How About a Nice Game of Chess? ---"
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let pwd = Hacker::new("reyedfim").crack_password();
        Ok(pwd)
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let pwd = Hacker::new("reyedfim").crack_password_advanced();
        Ok(pwd)
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {}
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "f97c354d");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "863dde27");
    }
}
