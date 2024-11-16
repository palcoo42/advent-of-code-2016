use super::action::Action;

#[derive(Debug, PartialEq)]
pub struct Document {
    actions: Vec<Action>,
}

impl Document {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter()
    }
}
