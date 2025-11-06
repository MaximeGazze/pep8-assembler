use std::{collections::HashMap, fmt::Display};

use crate::{lexer::Token, types::Pep8Word};

#[derive(Debug)]
pub enum Error {
    InvalidAddressTokenType(Token),
    InvalidAddrModeString(String),
    IllegalAddrMode(AddrMode),
    MalformedAddrMode,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAddressTokenType(token) => {
                write!(f, "invalid address token type: {token:?}")
            }
            Self::InvalidAddrModeString(value) => write!(f, "invalid addressing mode: {value}"),
            Self::IllegalAddrMode(mode) => write!(f, "illegal addressing mode: {mode}"),
            Self::MalformedAddrMode => write!(f, "addressing mode malformed"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq)]
pub enum AddrLocation {
    Memory(Pep8Word),
    Label(String),
}

impl AddrLocation {
    pub fn from_token(token: Token) -> Result<Self, Box<dyn std::error::Error>> {
        match token {
            Token::Char(value) => Ok(Self::Memory(Pep8Word::try_from(value)?)),
            Token::String(value) => Ok(Self::Memory(Pep8Word::try_from(value)?)),
            Token::Number(value) => Ok(Self::Memory(Pep8Word::new(value))),
            Token::Identifier(value) => Ok(Self::Label(value)),
            _ => Err(Box::new(Error::InvalidAddressTokenType(token))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddrMode {
    Immediate,
    Direct,
    Indirect,
    StackRelative,
    StackRelativeDeferred,
    Indexed,
    StackIndexed,
    StackIndexedDeferred,
}

impl Display for AddrMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Immediate => write!(f, "i"),
            Self::Direct => write!(f, "d"),
            Self::Indirect => write!(f, "n"),
            Self::StackRelative => write!(f, "s"),
            Self::StackRelativeDeferred => write!(f, "sf"),
            Self::Indexed => write!(f, "x"),
            Self::StackIndexed => write!(f, "sx"),
            Self::StackIndexedDeferred => write!(f, "sxf"),
        }
    }
}

impl AddrMode {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "i" => Ok(Self::Immediate),
            "d" => Ok(Self::Direct),
            "n" => Ok(Self::Indirect),
            "s" => Ok(Self::StackRelative),
            "sf" => Ok(Self::StackRelativeDeferred),
            "x" => Ok(Self::Indexed),
            "sx" => Ok(Self::StackIndexed),
            "sxf" => Ok(Self::StackIndexedDeferred),
            _ => Err(Error::InvalidAddrModeString(String::from(s))),
        }
    }

    pub fn as_byte_short(&self) -> Result<u8, Error> {
        match &self {
            Self::Immediate => Ok(0),
            Self::Indexed => Ok(1),
            _ => Err(Error::IllegalAddrMode(self.clone())),
        }
    }

    pub fn as_byte_long(&self) -> u8 {
        match &self {
            Self::Immediate => 0b000,
            Self::Direct => 0b001,
            Self::Indirect => 0b010,
            Self::StackRelative => 0b011,
            Self::StackRelativeDeferred => 0b100,
            Self::Indexed => 0b101,
            Self::StackIndexed => 0b110,
            Self::StackIndexedDeferred => 0b111,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    pub location: AddrLocation,
    pub mode: AddrMode,
}

impl Address {
    pub fn from_tokens_short(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens {
            [address_token] => Ok(Address {
                location: AddrLocation::from_token(address_token.clone())?,
                mode: AddrMode::Immediate,
            }),
            [address_token, Token::Comma, Token::Identifier(mode_value)] => {
                let mode = AddrMode::from_str(&mode_value)?;

                match mode {
                    AddrMode::Immediate | AddrMode::Indexed => Ok(Address {
                        location: AddrLocation::from_token(address_token.clone())?,
                        mode,
                    }),
                    _ => Err(Box::new(Error::IllegalAddrMode(mode))),
                }
            }
            _ => Err(Box::new(Error::MalformedAddrMode)),
        }
    }

    pub fn from_tokens_long(
        tokens: &[Token],
        legal_addressing_modes: &[AddrMode],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens {
            [address_token, Token::Comma, Token::Identifier(mode_value)] => {
                let mode = AddrMode::from_str(&mode_value)?;

                if legal_addressing_modes.contains(&mode) {
                    Ok(Address {
                        location: AddrLocation::from_token(address_token.clone())?,
                        mode,
                    })
                } else {
                    Err(Box::new(Error::IllegalAddrMode(mode)))
                }
            }
            _ => Err(Box::new(Error::MalformedAddrMode)),
        }
    }
}

#[derive(Debug)]
pub struct AddressTable {
    table: HashMap<String, Pep8Word>,
}

impl AddressTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Option<&Pep8Word> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: String, value: Pep8Word) -> Option<Pep8Word> {
        self.table.insert(key, value)
    }

    pub fn resolve(&self, address: &Address) -> Option<Pep8Word> {
        match &address.location {
            AddrLocation::Memory(memory_address) => Some(*memory_address),
            AddrLocation::Label(label) => self.get(&label).copied(),
        }
    }
}
