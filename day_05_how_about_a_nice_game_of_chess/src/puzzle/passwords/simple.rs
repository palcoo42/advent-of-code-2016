use crate::puzzle::door::DIGEST_LENGTH;

use super::password::{Password, PASSWORD_LENGTH};

#[derive(Default)]
pub struct SimplePassword {
    pwd: String,
}

impl Password for SimplePassword {
    fn new() -> Self {
        Self {
            pwd: String::with_capacity(PASSWORD_LENGTH),
        }
    }

    fn get(&self) -> String {
        String::from(&self.pwd)
    }

    fn update(&mut self, md5: &str) {
        self.pwd
            .push(md5.chars().nth(DIGEST_LENGTH).unwrap_or_else(|| {
                panic!(
                    "Failed to unwrap value from '{}' at index '{}'",
                    md5, DIGEST_LENGTH
                )
            }));
    }

    fn is_complete(&self) -> bool {
        self.pwd.len() == PASSWORD_LENGTH
    }
}
