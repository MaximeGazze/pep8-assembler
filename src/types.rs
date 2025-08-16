#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pep8Word(u16);

impl Pep8Word {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = s.as_bytes();

        let value = match bytes.len() {
            0 => 0,
            1 => bytes[0] as u16,
            2 => (bytes[0] as u16) << 8 | (bytes[1] as u16),
            _ => return Err(Box::from("string must be at most length 2")),
        };

        Ok(Self(value))
    }

    pub fn from_number_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}
