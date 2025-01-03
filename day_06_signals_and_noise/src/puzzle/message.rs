use std::collections::HashMap;

use super::signal::Signal;

pub struct Message {
    signals: Vec<Signal>,
}

impl Message {
    pub fn new(signals: Vec<Signal>) -> Self {
        Self { signals }
    }

    fn collect_characters_usages(&self) -> Vec<HashMap<char, usize>> {
        // Count usages of characters per column between all signals
        let length = self.get_columns_len();

        let mut usages = (0..length)
            .map(|_| HashMap::<char, usize>::new())
            .collect::<Vec<_>>();

        for signal in &self.signals {
            for i in 0..length {
                let c = signal
                    .nth(i)
                    .unwrap_or_else(|| panic!("Invalid index '{}' in signal '{:?}'", i, signal));

                let map = usages
                    .get_mut(i)
                    .unwrap_or_else(|| panic!("Invalid index '{}' in 'usages'", i));

                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
        }

        usages
    }

    pub fn find_message_max_occurrences(&self) -> String {
        // Count usages of characters within columns
        let usages = self.collect_characters_usages();

        // In every columns find character with most occurrences
        usages
            .iter()
            .map(|map| {
                let mut max_count = 0;
                let mut max_char = ' ';

                map.iter().for_each(|(c, count)| {
                    if *count > max_count {
                        max_count = *count;
                        max_char = *c;
                    }
                });

                max_char
            })
            .collect::<String>()
    }

    pub fn find_message_min_occurrences(&self) -> String {
        // Count usages of characters within columns
        let usages = self.collect_characters_usages();

        // In every columns find character with least occurrences
        usages
            .iter()
            .map(|map| {
                let mut min_count = usize::MAX;
                let mut min_char = ' ';

                map.iter().for_each(|(c, count)| {
                    if *count < min_count {
                        min_count = *count;
                        min_char = *c;
                    }
                });

                min_char
            })
            .collect::<String>()
    }

    fn get_columns_len(&self) -> usize {
        // All signals should have the same length
        let length = self.signals.first().expect("Signals are empty").len();

        if self.signals.iter().any(|s| s.len() != length) {
            panic!("Some of the signals have different length")
        }

        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_message() -> Message {
        let signals = vec![
            Signal::new("eedadn"),
            Signal::new("drvtee"),
            Signal::new("eandsr"),
            Signal::new("raavrd"),
            Signal::new("atevrs"),
            Signal::new("tsrnev"),
            Signal::new("sdttsa"),
            Signal::new("rasrtv"),
            Signal::new("nssdts"),
            Signal::new("ntnada"),
            Signal::new("svetve"),
            Signal::new("tesnvt"),
            Signal::new("vntsnd"),
            Signal::new("vrdear"),
            Signal::new("dvrsen"),
            Signal::new("enarar"),
        ];

        Message::new(signals)
    }

    #[test]
    fn test_find_message_max_occurrences() {
        let message = create_message();

        assert_eq!(message.find_message_max_occurrences(), "easter");
    }

    #[test]
    fn test_find_message_min_occurrences() {
        let message = create_message();

        assert_eq!(message.find_message_min_occurrences(), "advent");
    }
}
