use std::collections::BTreeSet;

use super::instruction::Recipient;

pub struct Bot {
    id: usize,
    chips: BTreeSet<usize>,
    low: Recipient,
    high: Recipient,
}

impl Bot {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            chips: BTreeSet::new(),
            low: Recipient::None,
            high: Recipient::None,
        }
    }

    pub fn record_instructions(&mut self, low: Recipient, high: Recipient) {
        self.low = low;
        self.high = high;
    }

    pub fn get_low_recipient(&self) -> Recipient {
        self.low.clone()
    }

    pub fn get_high_recipient(&self) -> Recipient {
        self.high.clone()
    }

    pub fn add_chip(&mut self, chip: usize) {
        self.chips.insert(chip);
    }

    pub fn get_low_chip(&mut self) -> usize {
        if self.chips.is_empty() {
            panic!("Chips are empty, bot: {}", self.id);
        }

        self.chips.pop_first().unwrap()
    }

    pub fn get_high_chip(&mut self) -> usize {
        if self.chips.is_empty() {
            panic!("Chips are empty, bot: {}", self.id);
        }

        self.chips.pop_last().unwrap()
    }

    pub fn get_nr_of_chips(&self) -> usize {
        self.chips.len()
    }
}
