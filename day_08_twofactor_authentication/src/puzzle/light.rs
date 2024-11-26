#[derive(Debug, Clone, PartialEq)]
pub enum Light {
    On,
    Off,
}

impl Light {
    pub fn to_char(&self) -> char {
        match self {
            Light::On => '#',
            Light::Off => ' ',
        }
    }
}
