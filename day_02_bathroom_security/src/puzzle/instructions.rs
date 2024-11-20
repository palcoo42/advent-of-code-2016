use super::direction::Direction;

#[derive(Debug, Default)]
pub struct Instructions {
    internal: Vec<Vec<Direction>>,
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            internal: Vec::new(),
        }
    }

    pub fn push(&mut self, instruction: Vec<Direction>) {
        self.internal.push(instruction);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<Direction>> {
        self.internal.iter()
    }

    pub fn len(&self) -> usize {
        self.internal.len()
    }

    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}
