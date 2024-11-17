use super::{action::Action, direction::Direction, document::Document, position::Position};

#[derive(Debug)]
pub struct City {
    position: Position,
    direction: Direction,
}

impl Default for City {
    fn default() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            direction: Direction::North,
        }
    }
}

impl City {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_blocks_to_hq(&mut self, document: &Document) -> u32 {
        for action in document.iter() {
            self.step(action);
        }

        Position::distance(&Position { x: 0, y: 0 }, &self.position)
    }

    fn step(&mut self, action: &Action) {
        match self.direction {
            Direction::North => match action {
                Action::Left(steps) => self.turn_west(*steps),
                Action::Right(steps) => self.turn_east(*steps),
            },
            Direction::East => match action {
                Action::Left(steps) => self.turn_north(*steps),
                Action::Right(steps) => self.turn_south(*steps),
            },
            Direction::South => match action {
                Action::Left(steps) => self.turn_east(*steps),
                Action::Right(steps) => self.turn_west(*steps),
            },
            Direction::West => match action {
                Action::Left(steps) => self.turn_south(*steps),
                Action::Right(steps) => self.turn_north(*steps),
            },
        };
    }

    #[inline]
    fn turn_north(&mut self, steps: u32) {
        self.position.y += steps as i32;
        self.direction = Direction::North;
    }

    #[inline]
    fn turn_east(&mut self, steps: u32) {
        self.position.x += steps as i32;
        self.direction = Direction::East;
    }

    #[inline]
    fn turn_south(&mut self, steps: u32) {
        self.position.y -= steps as i32;
        self.direction = Direction::South;
    }

    #[inline]
    fn turn_west(&mut self, steps: u32) {
        self.position.x -= steps as i32;
        self.direction = Direction::West;
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::action::Action;

    use super::*;

    #[test]
    fn test_count_blocks_to_hq_r2_l3() {
        let mut city = City::new();

        assert_eq!(
            city.count_blocks_to_hq(&Document::new(vec![Action::Right(2), Action::Left(3)])),
            5
        );
    }

    #[test]
    fn test_count_blocks_to_hq_r2_r2_r2() {
        let mut city = City::new();

        assert_eq!(
            city.count_blocks_to_hq(&Document::new(vec![
                Action::Right(2),
                Action::Right(2),
                Action::Right(2)
            ])),
            2
        );
    }

    #[test]
    fn test_count_blocks_to_hq_r5_l5_r5_r3() {
        let mut city = City::new();

        assert_eq!(
            city.count_blocks_to_hq(&Document::new(vec![
                Action::Right(5),
                Action::Left(5),
                Action::Right(5),
                Action::Right(3)
            ])),
            12
        );
    }
}
