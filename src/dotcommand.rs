use std::fmt::Display;

use crate::{
    address::{self, AddressTable},
    lexer::Token,
    types::{Pep8Byte, Pep8Word},
};

#[derive(Debug)]
pub enum Error {
    InvalidArguments(String),
    InvalidDotCommand(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArguments(message) => {
                write!(f, "invalid arguments for dot command: {message}")
            }
            Self::InvalidDotCommand(name) => {
                write!(f, "invalid dot command: {name}")
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum DotCommand {
    ADDRSS(String),
    ASCII(String),
    BLOCK(usize),
    // BURN,
    BYTE(Pep8Byte),
    END,
    // EQUATE,
    WORD(Pep8Word),
}

impl DotCommand {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        let [Token::DotCommand(dotcommand), other_tokens @ ..] = tokens else {
            return Err(Box::from("missing dot command token"));
        };

        match &dotcommand.to_uppercase()[..] {
            ".ADDRSS" => match other_tokens {
                [Token::Identifier(label)] => Ok(DotCommand::ADDRSS(label.clone())),
                _ => Err(Box::new(Error::InvalidArguments(String::from(
                    "label argument required",
                )))),
            },
            ".ASCII" => match other_tokens {
                [Token::String(value)] => Ok(Self::ASCII(value.clone())),
                _ => Err(Box::new(Error::InvalidArguments(String::from(
                    "string argument required",
                )))),
            },
            ".BLOCK" => match other_tokens {
                [Token::Number(value)] => Ok(Self::BLOCK(*value as usize)),
                _ => Err(Box::new(Error::InvalidArguments(String::from(
                    "number argument required",
                )))),
            },
            // ".BURN" => ,
            ".BYTE" => match other_tokens {
                [Token::Char(value)] => Ok(Self::BYTE(Pep8Byte::try_from(value)?)),
                [Token::Number(value)] => Ok(Self::BYTE(Pep8Byte::try_from(value)?)),
                [Token::String(value)] => Ok(Self::BYTE(Pep8Byte::try_from(value)?)),
                _ => Err(Box::new(Error::InvalidArguments(String::from(
                    "char, number or string argument required",
                )))),
            },
            ".END" => match other_tokens {
                [] => Ok(Self::END),
                _ => Err(Box::new(Error::InvalidArguments(String::from(
                    "no arguments expected",
                )))),
            },
            // ".EQUATE" => match other_tokens {
            //
            // },
            ".WORD" => match other_tokens {
                [Token::Char(value)] => Ok(Self::WORD(Pep8Word::try_from(value)?)),
                [Token::Number(value)] => Ok(Self::WORD(Pep8Word::try_from(value)?)),
                [Token::String(value)] => Ok(Self::WORD(Pep8Word::try_from(value)?)),
                _ => Err(Box::new(Error::InvalidArguments(String::from(
                    "char, number or string argument required",
                )))),
            },
            _ => Err(Box::new(Error::InvalidDotCommand(dotcommand.clone()))),
        }
    }

    pub fn as_bytes(&self, address_table: &AddressTable) -> Result<Vec<u8>, address::Error> {
        match self {
            Self::ADDRSS(value) => address_table
                .get(value)
                .ok_or(address::Error::UndefinedSymbol(value.clone()))
                .and_then(|word| Ok(word.as_bytes().to_vec())),
            Self::ASCII(value) => Ok(value.as_bytes().to_vec()),
            Self::BLOCK(size) => Ok(vec![0; *size]),
            // Self::BURN => ?, // WTF IS THIS LOL
            Self::BYTE(value) => Ok(vec![value.as_byte()]),
            Self::END => Ok(vec![]),
            // Self::EQUATE ,
            Self::WORD(value) => Ok(value.as_bytes().to_vec()),
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Self::ADDRSS(_) => 2,
            Self::ASCII(s) => s.len(), // FIXME this is clearly wrong
            Self::BLOCK(size) => *size,
            // Self::BURN => ?, // WTF IS THIS LOL
            Self::BYTE(_) => 1,
            Self::END => 1,
            // Self::EQUATE => 0,
            Self::WORD(_) => 2,
        }
    }
}
