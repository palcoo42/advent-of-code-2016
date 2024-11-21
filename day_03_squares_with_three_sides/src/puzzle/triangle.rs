#[derive(Debug, PartialEq)]
pub struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

impl Triangle {
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }

    pub fn is_valid(&self) -> bool {
        // In a valid triangle, the sum of any two sides must be larger than the remaining side
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(Triangle::new(3, 4, 5).is_valid());
    }

    #[test]
    fn test_is_invalid() {
        assert!(!Triangle::new(5, 10, 25).is_valid());
    }
}
