use std::collections::HashSet;

use super::{action::Action, direction::Direction, document::Document, position::Position};

#[derive(Debug)]
pub struct City {
    position: Position,
    direction: Direction,
    visited: HashSet<Position>,
    stop_on_intersection: bool,
    intersected_position: Position,
}

impl Default for City {
    fn default() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            direction: Direction::North,
            visited: HashSet::new(),
            stop_on_intersection: false,
            intersected_position: Position { x: 0, y: 0 },
        }
    }
}

impl City {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_blocks_to_hq(&mut self, document: &Document) -> u32 {
        // Reset members
        self.position = Position { x: 0, y: 0 };
        self.direction = Direction::North;
        self.visited = HashSet::new();
        self.stop_on_intersection = false;

        self.travel(document);

        Position::distance(&Position { x: 0, y: 0 }, &self.position)
    }

    pub fn find_first_intersection(&mut self, document: &Document) -> u32 {
        // Reset members
        self.position = Position { x: 0, y: 0 };
        self.direction = Direction::North;
        self.visited = HashSet::new();
        self.stop_on_intersection = true;

        self.visited.insert(Position { x: 0, y: 0 });
        self.travel(document);

        Position::distance(&Position { x: 0, y: 0 }, &self.intersected_position)
    }

    fn travel(&mut self, document: &Document) {
        for action in document.iter() {
            if !self.step(action) {
                break;
            }
        }
    }

    fn step(&mut self, action: &Action) -> bool {
        let positions = match self.direction {
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

        if self.stop_on_intersection {
            return self.visit(positions);
        }

        // Continue
        true
    }

    #[inline]
    fn turn_north(&mut self, steps: u32) -> Vec<Position> {
        let locations = (self.position.y + 1..=self.position.y + steps as i32)
            .map(|y| Position {
                x: self.position.x,
                y,
            })
            .collect::<Vec<_>>();

        self.position.y += steps as i32;
        self.direction = Direction::North;

        locations
    }

    #[inline]
    fn turn_east(&mut self, steps: u32) -> Vec<Position> {
        let locations = (self.position.x + 1..=self.position.x + steps as i32)
            .map(|x| Position {
                x,
                y: self.position.y,
            })
            .collect::<Vec<_>>();

        self.position.x += steps as i32;
        self.direction = Direction::East;

        locations
    }

    #[inline]
    fn turn_south(&mut self, steps: u32) -> Vec<Position> {
        let locations = (self.position.y - steps as i32..self.position.y)
            .map(|y| Position {
                x: self.position.x,
                y,
            })
            .collect::<Vec<_>>();

        self.position.y -= steps as i32;
        self.direction = Direction::South;

        locations
    }

    #[inline]
    fn turn_west(&mut self, steps: u32) -> Vec<Position> {
        let locations = (self.position.x - steps as i32..self.position.x)
            .map(|x| Position {
                x,
                y: self.position.y,
            })
            .collect::<Vec<_>>();

        self.position.x -= steps as i32;
        self.direction = Direction::West;

        locations
    }

    fn visit(&mut self, positions: Vec<Position>) -> bool {
        // Set all positions as visited
        for pos in positions {
            if !self.visited.insert(pos.clone()) {
                // Position is already visited -> stop here
                self.intersected_position = pos;
                return false;
            }
        }

        true
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

    #[test]
    fn test_find_first_intersection() {
        let mut city = City::new();

        assert_eq!(
            city.find_first_intersection(&Document::new(vec![
                Action::Right(8),
                Action::Right(4),
                Action::Right(4),
                Action::Right(8)
            ])),
            4
        );
    }

    #[test]
    fn test_turn_north_positive_position() {
        let mut city = City::new();
        city.position.y = 10;

        assert_eq!(
            city.turn_north(4),
            vec![
                Position { x: 0, y: 11 },
                Position { x: 0, y: 12 },
                Position { x: 0, y: 13 },
                Position { x: 0, y: 14 }
            ]
        );
        assert_eq!(city.position, Position { x: 0, y: 14 });
        assert_eq!(city.direction, Direction::North);
    }

    #[test]
    fn test_turn_north_negative_position() {
        let mut city = City::new();
        city.position.y = -10;

        assert_eq!(
            city.turn_north(4),
            vec![
                Position { x: 0, y: -9 },
                Position { x: 0, y: -8 },
                Position { x: 0, y: -7 },
                Position { x: 0, y: -6 }
            ]
        );
        assert_eq!(city.position, Position { x: 0, y: -6 });
        assert_eq!(city.direction, Direction::North);
    }

    #[test]
    fn test_turn_east_positive_position() {
        let mut city = City::new();
        city.position.x = 10;

        assert_eq!(
            city.turn_east(4),
            vec![
                Position { x: 11, y: 0 },
                Position { x: 12, y: 0 },
                Position { x: 13, y: 0 },
                Position { x: 14, y: 0 }
            ]
        );
        assert_eq!(city.position, Position { x: 14, y: 0 });
        assert_eq!(city.direction, Direction::East);
    }

    #[test]
    fn test_turn_east_negative_position() {
        let mut city = City::new();
        city.position.x = -10;

        assert_eq!(
            city.turn_east(4),
            vec![
                Position { x: -9, y: 0 },
                Position { x: -8, y: 0 },
                Position { x: -7, y: 0 },
                Position { x: -6, y: 0 }
            ]
        );
        assert_eq!(city.position, Position { x: -6, y: 0 });
        assert_eq!(city.direction, Direction::East);
    }

    #[test]
    fn test_turn_south_positive_position() {
        let mut city = City::new();
        city.position.y = 10;

        assert_eq!(
            city.turn_south(4),
            vec![
                Position { x: 0, y: 6 },
                Position { x: 0, y: 7 },
                Position { x: 0, y: 8 },
                Position { x: 0, y: 9 }
            ]
        );
        assert_eq!(city.position, Position { x: 0, y: 6 });
        assert_eq!(city.direction, Direction::South);
    }

    #[test]
    fn test_turn_south_negative_position() {
        let mut city = City::new();
        city.position.y = -10;

        assert_eq!(
            city.turn_south(4),
            vec![
                Position { x: 0, y: -14 },
                Position { x: 0, y: -13 },
                Position { x: 0, y: -12 },
                Position { x: 0, y: -11 }
            ]
        );
        assert_eq!(city.position, Position { x: 0, y: -14 });
        assert_eq!(city.direction, Direction::South);
    }

    #[test]
    fn test_turn_west_positive_position() {
        let mut city = City::new();
        city.position.x = 10;

        assert_eq!(
            city.turn_west(4),
            vec![
                Position { x: 6, y: 0 },
                Position { x: 7, y: 0 },
                Position { x: 8, y: 0 },
                Position { x: 9, y: 0 }
            ]
        );
        assert_eq!(city.position, Position { x: 6, y: 0 });
        assert_eq!(city.direction, Direction::West);
    }

    #[test]
    fn test_turn_west_negative_position() {
        let mut city = City::new();
        city.position.x = -10;

        assert_eq!(
            city.turn_west(4),
            vec![
                Position { x: -14, y: 0 },
                Position { x: -13, y: 0 },
                Position { x: -12, y: 0 },
                Position { x: -11, y: 0 }
            ]
        );
        assert_eq!(city.position, Position { x: -14, y: 0 });
        assert_eq!(city.direction, Direction::West);
    }
}
