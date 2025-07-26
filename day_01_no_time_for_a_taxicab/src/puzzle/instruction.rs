#[derive(Debug)]
pub enum Instruction {
    Left { steps: usize },
    Right { steps: usize },
}
