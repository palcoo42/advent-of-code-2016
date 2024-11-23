use crate::puzzle::door::DIGEST_LENGTH;

use super::password::{Password, PASSWORD_LENGTH};

// Filler in the password name at positions which were not filled yet
const FILLER: char = 'x';

pub struct ComplexPassword {
    pwd: [char; PASSWORD_LENGTH],
}

impl Password for ComplexPassword {
    fn new() -> Self {
        Self {
            pwd: [FILLER; PASSWORD_LENGTH],
        }
    }

    fn get(&self) -> String {
        self.pwd.iter().collect()
    }

    fn update(&mut self, md5: &str) {
        // 6th - position
        // 7th - value at this position
        let position = md5
            .chars()
            .nth(DIGEST_LENGTH)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to unwrap position from '{}' at index '{}'",
                    md5, DIGEST_LENGTH
                )
            })
            .to_digit(16)
            .unwrap_or_else(|| panic!("Failed to convert position to u32"));

        // Update password only if position is within password length
        if (position as usize) < PASSWORD_LENGTH {
            // Do not overwrite already filled position
            if self.pwd[position as usize] != FILLER {
                return;
            }

            let value = md5.chars().nth(DIGEST_LENGTH + 1).unwrap_or_else(|| {
                panic!(
                    "Failed to unwrap value from '{}' at index '{}'",
                    md5,
                    DIGEST_LENGTH + 1,
                )
            });

            self.pwd[position as usize] = value;
        }
    }

    fn is_complete(&self) -> bool {
        // Password is complete if all FILLERs are replaced
        self.pwd.iter().all(|b| *b != FILLER)
    }
}
