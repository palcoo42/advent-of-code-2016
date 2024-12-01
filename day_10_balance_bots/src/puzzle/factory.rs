use std::collections::{HashMap, VecDeque};

use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{
    bot::Bot,
    instruction::{Instruction, Recipient},
};

#[derive(Default)]
pub struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, Vec<usize>>,
}

impl Factory {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        // Populate factory with bots
        let mut bots = HashMap::new();

        for instruction in instructions {
            match instruction {
                Instruction::Assign(id, chip) => {
                    let bot = bots.entry(id).or_insert_with(|| Bot::new(id));
                    bot.add_chip(chip);
                }
                Instruction::Give(id, low, high) => {
                    let bot = bots.entry(id).or_insert_with(|| Bot::new(id));
                    bot.record_instructions(low, high);
                }
            }
        }

        Self {
            bots,
            outputs: HashMap::new(),
        }
    }

    pub fn find_bot_who_compares_chips(&mut self, chips: &[usize]) -> Result<usize, PuzzleError> {
        if chips.len() != 2 {
            return Err(PuzzleError::GenericError(format!(
                "Exactly two chips are expected '{:?}'",
                chips
            )));
        }

        // Sort chips for easier comparison
        let mut chips = chips.iter().collect::<Vec<_>>();
        chips.sort();

        // Run factory
        let mut remaining_bots = self.bots.keys().cloned().collect::<VecDeque<_>>();

        while let Some(bot_id) = remaining_bots.pop_front() {
            // Find current bot and check if it has two chips
            let bot = self
                .bots
                .get_mut(&bot_id)
                .unwrap_or_else(|| panic!("Failed to find bot '{}'", bot_id));

            if bot.get_nr_of_chips() == 2 {
                // We can distribute chips
                let low_chip = bot.get_low_chip();
                let high_chip = bot.get_high_chip();
                let low_recipient = bot.get_low_recipient();
                let high_recipient = bot.get_high_recipient();

                self.distribute_chip(low_recipient, low_chip);
                self.distribute_chip(high_recipient, high_chip);

                if low_chip == *chips[0] && high_chip == *chips[1] {
                    return Ok(bot_id);
                }
            } else {
                // We need to wait for both chips to be available
                remaining_bots.push_back(bot_id);
            }
        }

        Err(PuzzleError::GenericError(String::from(
            "Failed to find bot",
        )))
    }

    fn distribute_chip(&mut self, recipient: Recipient, chip: usize) {
        match recipient {
            // Move chip to the 'bot'
            Recipient::Bot(id) => {
                let bot_rcv = self
                    .bots
                    .get_mut(&id)
                    .unwrap_or_else(|| panic!("Failed to retrieve bot '{}'", id));

                bot_rcv.add_chip(chip);
            }

            // Move chip to the 'output'
            Recipient::Output(id) => {
                let output = self.outputs.entry(id).or_default();
                output.push(chip);
            }

            // Not expected at this point
            Recipient::None => {
                panic!("Unexpected recipient None")
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::puzzle::instruction::Recipient;

    use super::*;

    fn create_factory() -> Factory {
        let instructions = vec![
            Instruction::Assign(2, 5),
            Instruction::Give(2, Recipient::Bot(1), Recipient::Bot(0)),
            Instruction::Assign(1, 3),
            Instruction::Give(1, Recipient::Output(1), Recipient::Bot(0)),
            Instruction::Give(0, Recipient::Output(2), Recipient::Output(0)),
            Instruction::Assign(2, 2),
        ];
        Factory::new(instructions)
    }

    #[test]
    fn test_find_bot_who_compares_chips() {
        let mut factory = create_factory();

        let result = factory.find_bot_who_compares_chips(&[5, 2]);

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), 2);
    }
}
