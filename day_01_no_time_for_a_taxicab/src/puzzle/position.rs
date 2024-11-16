#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance(a: &Position, b: &Position) -> u32 {
        (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(
            Position::distance(&Position { x: 0, y: 0 }, &Position { x: 2, y: 3 }),
            5
        );
        assert_eq!(
            Position::distance(&Position { x: -2, y: 1 }, &Position { x: 0, y: 0 }),
            3
        );
        assert_eq!(
            Position::distance(&Position { x: -2, y: -2 }, &Position { x: 2, y: 2 }),
            8
        );
    }
}
