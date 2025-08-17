use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pep8Word(u16);

impl Pep8Word {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl TryFrom<&char> for Pep8Word {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        if value.is_ascii() {
            Ok(Self::new(*value as u16))
        } else {
            Err(Box::from("invalid ascii character"))
        }
    }
}

impl TryFrom<&[u8]> for Pep8Word {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let word = match value.len() {
            0 => 0,
            1 => value[0] as u16,
            2 => (value[0] as u16) << 8 | (value[1] as u16),
            _ => return Err(Box::from("bytes must be at most length 2")),
        };

        Ok(Self(word))
    }
}

impl TryFrom<&String> for Pep8Word {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl TryFrom<&str> for Pep8Word {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl From<&u16> for Pep8Word {
    fn from(value: &u16) -> Self {
        Self(*value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pep8Byte(u8);

impl Pep8Byte {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn as_byte(self) -> u8 {
        self.0
    }
}

impl Add for Pep8Byte {
    type Output = Pep8Byte;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<&Pep8Byte> for Pep8Byte {
    type Output = Pep8Byte;

    fn add(self, rhs: &Pep8Byte) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl PartialOrd for Pep8Byte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pep8Byte {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl TryFrom<&char> for Pep8Byte {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        if value.is_ascii() {
            Ok(Self::new(*value as u8))
        } else {
            Err(Box::from("invalid ascii character"))
        }
    }
}

impl TryFrom<&String> for Pep8Byte {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();

        let value = match bytes.len() {
            0 => 0,
            1 => bytes[0] as u8,
            _ => return Err(Box::from("string must be at most length 1")),
        };

        Ok(Self(value))
    }
}

impl From<&u16> for Pep8Byte {
    fn from(value: &u16) -> Self {
        Self(*value as u8)
    }
}
