#[derive(Debug, Clone, PartialEq)]
pub enum Recipient {
    Bot(usize),
    Output(usize),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Assign(usize, usize),              // bot id, value
    Give(usize, Recipient, Recipient), // bot id, recipient, recipient
}
