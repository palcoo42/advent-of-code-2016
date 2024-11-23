use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::passwords::password::Password;

// Digest length which is analyzed
pub const DIGEST_LENGTH: usize = 5;

// Digest code when we have found a solution
const DIGEST_CODE: &str = "00000";

pub struct Door {
    door_id: String,
}

impl Door {
    pub fn new(door_id: &str) -> Self {
        Self {
            door_id: String::from(door_id),
        }
    }

    pub fn decode_password<T>(&self) -> Result<String, PuzzleError>
    where
        T: Password,
    {
        // Password is generic to support different algorithms
        let mut password = T::new();

        // Current number used for MD5 calculation
        let mut number = 0_usize;

        while !password.is_complete() && number < usize::MAX {
            // Calculate MD5 for current code
            let code = format!("{}{}", self.door_id, number);
            let md5 = Self::calc_md5(&code);

            // If we have found solution hash update password
            if md5.starts_with(DIGEST_CODE) {
                password.update(&md5);

                if password.is_complete() {
                    return Ok(password.get());
                }
            }

            // Go to the next value
            number += 1;
        }

        Err(PuzzleError::GenericError(format!(
            "Failed to decode password for door '{}'",
            self.door_id
        )))
    }

    fn calc_md5(value: &str) -> String {
        format!("{:?}", md5::compute(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::passwords::{complex::ComplexPassword, simple::SimplePassword};

    use super::*;

    #[test]
    #[ignore = "Long running test"]
    fn test_decode_password_simple() {
        let door = Door::new("abc");
        let result = door.decode_password::<SimplePassword>();

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), "18f47a30");
    }

    #[test]
    #[ignore = "Long running test"]
    fn test_decode_password_complex() {
        let door = Door::new("abc");
        let result = door.decode_password::<ComplexPassword>();

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), "05ace8e3");
    }

    #[test]
    fn test_calc_md5() {
        assert_eq!(Door::calc_md5("abc1"), "23734cd52ad4a4fb877d8a1e26e5df5f");
        assert_eq!(
            Door::calc_md5("abc3231929"),
            "00000155f8105dff7f56ee10fa9b9abd"
        );
        assert_eq!(
            Door::calc_md5("abc5017308"),
            "000008f82c5b3924a1ecbebf60344e00"
        );
        assert_eq!(
            Door::calc_md5("abc5278568"),
            "00000f9a2c309875e05c5a5d09f1b8c4"
        );
    }
}
