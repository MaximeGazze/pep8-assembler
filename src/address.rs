use std::collections::HashMap;

use crate::{
    lexer::{Token, TokenType},
    types::Pep8Word,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AddrLocation {
    Memory(Pep8Word),
    Label(String),
}

impl AddrLocation {
    pub fn from_token(token: &Token) -> Result<Self, Box<dyn std::error::Error>> {
        match token.kind {
            TokenType::Char | TokenType::String => {
                Ok(AddrLocation::Memory(Pep8Word::from_str(&token.value)?))
            }
            TokenType::Number => Ok(AddrLocation::Memory(Pep8Word::from_number_str(
                &token.value,
            )?)),
            TokenType::Identifier => Ok(AddrLocation::Label(token.value.clone())),
            _ => Err(Box::from("invalid address token")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl AddrMode {
    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s {
            "i" => Ok(Self::Immediate),
            "d" => Ok(Self::Direct),
            "n" => Ok(Self::Indirect),
            "s" => Ok(Self::StackRelative),
            "sf" => Ok(Self::StackRelativeDeferred),
            "x" => Ok(Self::Indexed),
            "sx" => Ok(Self::StackIndexed),
            "sxf" => Ok(Self::StackIndexedDeferred),
            _ => Err(Box::from("invalid addressing mode string")),
        }
    }

    pub fn as_byte_short(&self) -> Result<u8, Box<dyn std::error::Error>> {
        match &self {
            Self::Immediate => Ok(0),
            Self::Indexed => Ok(1),
            _ => Err(Box::from("invalid register type")),
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
                location: AddrLocation::from_token(address_token)?,
                mode: AddrMode::Immediate,
            }),
            [address_token, comma_token, mode_token] if comma_token.kind == TokenType::Comma => {
                let mode = AddrMode::from_str(&mode_token.value)?;

                match mode {
                    AddrMode::Immediate | AddrMode::Indexed => Ok(Address {
                        location: AddrLocation::from_token(address_token)?,
                        mode: AddrMode::from_str(&mode_token.value)?,
                    }),
                    _ => Err(Box::from("illegal addressing mode")),
                }
            }
            _ => Err(Box::from("malformed addressing mode")),
        }
    }

    pub fn from_tokens_long(
        tokens: &[Token],
        legal_addressing_modes: &[AddrMode],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens {
            [address_token, comma_token, mode_token] if comma_token.kind == TokenType::Comma => {
                let mode = AddrMode::from_str(&mode_token.value)?;

                if legal_addressing_modes.contains(&mode) {
                    Ok(Address {
                        location: AddrLocation::from_token(address_token)?,
                        mode,
                    })
                } else {
                    Err(Box::from("illegal addressing mode"))
                }
            }
            _ => Err(Box::from("malformed addressing mode")),
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
