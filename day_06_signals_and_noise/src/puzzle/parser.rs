use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{message::Message, signal::Signal};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Message, PuzzleError> {
        let signals = lines
            .iter()
            .map(|line| Signal::new(line))
            .collect::<Vec<_>>();

        Ok(Message::new(signals))
    }
}
