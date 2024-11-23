use advent_of_code::puzzles::puzzle_error::PuzzleError;

// Length of the password
const PASSWORD_LENGTH: usize = 8;

// Digest length which is analyzed
const DIGEST_LENGTH: usize = 5;

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

    pub fn decode_password(&self) -> Result<String, PuzzleError> {
        let mut pwd = String::with_capacity(PASSWORD_LENGTH);
        let mut number = 0_usize;

        while pwd.len() < PASSWORD_LENGTH && number < usize::MAX {
            // Calculate MD5 for current code
            let code = format!("{}{}", self.door_id, number);
            let md5 = Self::calc_md5(&code);

            // If we have a valid solution append 6th character to the code
            if md5.starts_with(DIGEST_CODE) {
                pwd.push(md5.chars().nth(DIGEST_LENGTH).unwrap_or_else(|| {
                    panic!(
                        "Failed to unwrap {}th value from '{}'",
                        DIGEST_LENGTH + 1,
                        md5
                    )
                }));

                if pwd.len() == PASSWORD_LENGTH {
                    return Ok(pwd);
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
    use super::*;

    #[test]
    #[ignore = "Long running test"]
    fn test_decode_password() {
        let door = Door::new("abc");
        let result = door.decode_password();

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), "18f47a30");
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
