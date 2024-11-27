use advent_of_code::puzzles::puzzle_error::PuzzleError;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<String, PuzzleError> {
        if lines.len() > 1 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Input file should contain only 1 line, but '{}' found",
                lines.len()
            )));
        }

        Ok(String::from(lines[0]))
    }
}
