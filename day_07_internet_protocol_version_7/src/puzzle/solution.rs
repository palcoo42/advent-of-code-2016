use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::ipv7::IpV7;

pub struct Solution {
    addresses: Vec<String>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 7: Internet Protocol Version 7 ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_07.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_07.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.addresses = lines;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let tls_count = self
            .addresses
            .iter()
            .filter(|url| Self::supports_tls(url))
            .count();

        Ok(tls_count.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let ssl_count = self
            .addresses
            .iter()
            .filter(|url| Self::supports_ssl(url))
            .count();

        Ok(ssl_count.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            addresses: Vec::new(),
        }
    }

    fn supports_tls(address: &str) -> bool {
        // Traverse string only once and collect all positions
        let ipv7 = IpV7::new(address);

        // If inside part contains ABBA TLS is not supported
        for inside in &ipv7.inside {
            if Self::has_abba(inside) {
                return false;
            }
        }

        // If outside part contains ABBA TLS is supported
        for outside in &ipv7.outside {
            if Self::has_abba(outside) {
                return true;
            }
        }

        false
    }

    fn has_abba(word: &str) -> bool {
        let windows = word.as_bytes().windows(4);

        for bytes in windows {
            if bytes[0] == bytes[3] && bytes[1] == bytes[2] && bytes[0] != bytes[1] {
                return true;
            }
        }

        false
    }

    fn supports_ssl(address: &str) -> bool {
        // Traverse string only once and collect all positions
        let ipv7 = IpV7::new(address);

        // Inside part shall contain ABA and outide BAB (or vice versa)
        let abas = ipv7
            .inside
            .iter()
            .flat_map(|url| Self::get_aba(url))
            .collect::<Vec<_>>();

        let babs = ipv7
            .outside
            .iter()
            .flat_map(|url| Self::get_aba(url))
            .collect::<Vec<_>>();

        // At least one ABA has to match BAA
        for aba in &abas {
            let aba_bytes = aba.as_bytes();

            for bab in &babs {
                let bab_bytes = bab.as_bytes();

                if aba_bytes[0] == bab_bytes[1] && aba_bytes[1] == bab_bytes[0] {
                    return true;
                }
            }
        }

        false
    }

    // Returns only ABA ranges
    fn get_aba(word: &str) -> Vec<&str> {
        let mut abas = Vec::new();
        let windows = word.as_bytes().windows(3);

        for (index, bytes) in windows.enumerate() {
            if bytes[0] == bytes[2] && bytes[0] != bytes[1] {
                abas.push(&word[index..index + 3]);
            }
        }

        abas
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "115");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "231");
    }

    #[test]
    fn test_has_abba() {
        assert!(Solution::has_abba("abba"));
        assert!(Solution::has_abba("oxyyxo"));
    }

    #[test]
    fn test_does_not_have_abba() {
        assert!(!Solution::has_abba("abca"));
        assert!(!Solution::has_abba("abbc"));
        assert!(!Solution::has_abba("aaaa"));
    }

    #[test]
    fn test_supports_tls() {
        assert!(Solution::supports_tls("abba[mnop]qrst"));
        assert!(Solution::supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_does_not_supports_tls() {
        assert!(!Solution::supports_tls("abcd[bddb]xyyx"));
        assert!(!Solution::supports_tls("aaaa[qwer]tyui"));
        assert!(!Solution::supports_tls(
            "itgslvpxoqqakli[arktzcssgkxktejbno]wsgkbwwtbmfnddt[zblrboqsvezcgfmfvcz]iwyhyatqetsreeyhh"
        ));
    }

    #[test]
    fn test_has_aba() {
        assert_eq!(Solution::get_aba("aba"), vec!["aba"]);
        assert_eq!(Solution::get_aba("abcaba"), vec!["aba"]);
        assert_eq!(Solution::get_aba("abcabaxy"), vec!["aba"]);
    }

    #[test]
    fn test_does_not_have_aba() {
        let empty: Vec<&str> = Vec::new();

        assert_eq!(Solution::get_aba("abc"), empty);
        assert_eq!(Solution::get_aba("abb"), empty);
        assert_eq!(Solution::get_aba("aaa"), empty);
    }

    #[test]
    fn test_supports_ssl() {
        assert!(Solution::supports_ssl("aba[bab]xyz"));
        assert!(Solution::supports_ssl("aaa[kek]eke"));
        assert!(Solution::supports_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn test_does_not_supports_ssl() {
        assert!(!Solution::supports_ssl("xyx[xyx]xyx"));
        assert!(!Solution::supports_ssl("aaaa[qwer]tyui"));
    }
}
