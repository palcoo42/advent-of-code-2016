use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{direction::Direction, instructions::Instructions};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[String]) -> Result<Instructions, PuzzleError> {
        let mut instructions = Instructions::new();

        for line in lines {
            let directions = Self::line_to_directions(&line)?;
            instructions.push(directions);
        }

        Ok(instructions)
    }

    fn line_to_directions(line: &str) -> Result<Vec<Direction>, PuzzleError> {
        let mut directions = Vec::with_capacity(line.len());

        for c in line.chars() {
            let direction = match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'D' => Direction::Down,
                'U' => Direction::Up,
                _ => {
                    return Err(PuzzleError::InvalidContentError(format!(
                        "Unsupported direction '{c}' in line '{line}'"
                    )))
                }
            };

            directions.push(direction);
        }

        Ok(directions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_directions() {
        assert_eq!(
            Parser::line_to_directions("LRUD").unwrap(),
            vec![
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down
            ]
        )
    }
}
