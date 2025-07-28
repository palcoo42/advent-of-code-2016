use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    messages: Vec<Vec<char>>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 6: Signals and Noise ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_06.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_06.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.messages = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let message = Self::corrected_message(&self.messages, false);
        Ok(message.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let message = Self::corrected_message(&self.messages, true);
        Ok(message.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    fn corrected_message(messages: &[Vec<char>], modified_repetition_code: bool) -> String {
        let mut message = String::with_capacity(messages.len());

        // For every column count occurences
        for c in 0..messages[0].len() {
            let mut counts: HashMap<char, usize> = HashMap::new();

            for message in messages {
                *counts.entry(message[c]).or_insert(0) += 1;
            }

            // Find max occurence
            let (pwd, _count) = if modified_repetition_code {
                counts
                    .iter()
                    .min_by(|a, b| a.1.cmp(b.1))
                    .expect("Failed to find min")
            } else {
                counts
                    .iter()
                    .max_by(|a, b| a.1.cmp(b.1))
                    .expect("Failed to find max")
            };

            message.push(*pwd);
        }

        message
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "kqsdmzft");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "tpooccyo");
    }

    fn get_messages() -> Vec<Vec<char>> {
        vec![
            "eedadn".chars().collect(),
            "drvtee".chars().collect(),
            "eandsr".chars().collect(),
            "raavrd".chars().collect(),
            "atevrs".chars().collect(),
            "tsrnev".chars().collect(),
            "sdttsa".chars().collect(),
            "rasrtv".chars().collect(),
            "nssdts".chars().collect(),
            "ntnada".chars().collect(),
            "svetve".chars().collect(),
            "tesnvt".chars().collect(),
            "vntsnd".chars().collect(),
            "vrdear".chars().collect(),
            "dvrsen".chars().collect(),
            "enarar".chars().collect(),
        ]
    }

    #[test]
    fn test_corrected_message() {
        let messages = get_messages();
        assert_eq!(&Solution::corrected_message(&messages, false), "easter");
    }

    #[test]
    fn test_corrected_message_modified_repetition_code() {
        let messages = get_messages();
        assert_eq!(&Solution::corrected_message(&messages, true), "advent");
    }
}
