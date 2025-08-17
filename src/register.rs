#[derive(Debug)]
pub enum Register {
    Accumulator,
    IndexRegister,
}

impl Register {
    pub fn as_byte(&self) -> u8 {
        match self {
            Self::Accumulator => 0,
            Self::IndexRegister => 1,
        }
    }
}
