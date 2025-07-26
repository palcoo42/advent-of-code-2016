use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::puzzle::direction::Direction;

// Internal type to hold direction to next value mapping
type DirectionMapping<T> = HashMap<Direction, T>;

// Generic mapping of key to new positions after movement
pub struct KeypadMapping<T>
where
    T: Debug + Clone + ToString + PartialEq + Eq + Hash,
{
    keys: HashMap<T, DirectionMapping<T>>,
}

impl<T> KeypadMapping<T>
where
    T: Debug + Clone + ToString + PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: T, directions: &[(Direction, T)]) {
        let mut mapping = DirectionMapping::new();

        for (dir, value) in directions {
            mapping.insert(dir.clone(), value.clone());
        }

        self.keys.insert(key, mapping);
    }

    fn get(&self, key: &T, direction: &Direction) -> Option<&T> {
        self.keys
            .get(key)
            .and_then(|mapping| mapping.get(direction))
    }

    fn get_code_one(&self, position: T, directions: &[Direction]) -> T {
        let mut code = position.clone();

        for dir in directions {
            code = self.get(&code, dir).cloned().unwrap_or_else(|| {
                panic!("Failed to find direction '{dir:?}' for code '{code:?}'")
            });
        }

        code
    }

    pub fn get_code(&self, starting: T, directions: &[Vec<Direction>]) -> String {
        let mut code = String::with_capacity(directions.len());
        let mut position = starting.clone();

        for dirs in directions {
            position = self.get_code_one(position, dirs);
            code.push_str(&position.to_string());
        }

        code
    }
}
