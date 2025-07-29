use std::sync::LazyLock;

use regex::Regex;

// Compile regex only once for all IpV7 instances
static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([^\[\]]+)|\[(.*?)\]").expect("Invalid regex"));

#[derive(Debug, PartialEq)]
pub struct IpV7<'a> {
    pub outside: Vec<&'a str>,
    pub inside: Vec<&'a str>,
}

impl<'a> IpV7<'a> {
    pub fn new(address: &'a str) -> Self {
        let (outside, inside) = Self::extract(address);

        Self { outside, inside }
    }

    fn extract(address: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
        let mut outside = Vec::new();
        let mut inside = Vec::new();

        for capture in REGEX.captures_iter(address) {
            if let Some(out) = capture.get(1) {
                outside.push(out.as_str());
            }

            if let Some(ins) = capture.get(2) {
                inside.push(ins.as_str());
            }
        }

        (outside, inside)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            IpV7::new(
                "abpxdcnbqeoeiidhpt[zpwzuygklghkvrzsogw]mdmjoojzrwdqcywsxd[jbxptisjyvgicpqnw]aanbeosfyeptpuzmrz[pasvleayajolpwhj]hsbidwxbtlfdmsahbu"
            ),
            IpV7 {
                outside: vec![
                    "abpxdcnbqeoeiidhpt",
                    "mdmjoojzrwdqcywsxd",
                    "aanbeosfyeptpuzmrz",
                    "hsbidwxbtlfdmsahbu"
                ],
                inside: vec![
                    "zpwzuygklghkvrzsogw",
                    "jbxptisjyvgicpqnw",
                    "pasvleayajolpwhj"
                ]
            }
        )
    }
}
