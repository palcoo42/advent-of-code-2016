use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::ip_v7::Ipv7;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Ipv7>, PuzzleError> {
        let ipv7 = lines
            .iter()
            .map(|&line| Ipv7::new(line))
            .collect::<Vec<_>>();

        Ok(ipv7)
    }
}
