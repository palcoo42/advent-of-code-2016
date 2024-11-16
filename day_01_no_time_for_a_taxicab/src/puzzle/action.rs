#[derive(Debug, PartialEq)]
pub enum Action {
    Left(u32),
    Right(u32),
}

impl Action {
    pub fn new(action: &str, steps: u32) -> Self {
        match action {
            "R" => Action::Right(steps),
            "L" => Action::Left(steps),
            _ => panic!("Unsupported action '{}'", action),
        }
    }
}
