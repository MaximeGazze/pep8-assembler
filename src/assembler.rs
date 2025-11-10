use std::fmt::Display;

use crate::{
    address::AddressTable, dotcommand::DotCommand, instruction::Instruction, lexer::Token,
    types::Pep8Word,
};

#[derive(Debug)]
pub enum Error {
    TokensEmpty,
    InvalidTokenType,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokensEmpty => write!(f, "tokens is empty"),
            Self::InvalidTokenType => write!(f, "invalid token type"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Statement {
    Instruction(Instruction),
    DotCommand(DotCommand),
}

impl Statement {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens.first() {
            None => Err(Box::new(Error::TokensEmpty)),
            Some(token) => match token {
                Token::Identifier(_) => Ok(Self::Instruction(Instruction::from_tokens(tokens)?)),
                Token::DotCommand(_) => Ok(Self::DotCommand(DotCommand::from_tokens(tokens)?)),
                _ => Err(Box::new(Error::InvalidTokenType)),
            },
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Self::Instruction(instruction) => instruction.byte_size(),
            Self::DotCommand(dotcommand) => dotcommand.byte_size(),
        }
    }

    pub fn as_bytes(
        &self,
        address_table: &AddressTable,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self {
            Self::Instruction(instruction) => instruction.as_bytes(address_table),
            Self::DotCommand(dotcommand) => Ok(dotcommand.as_bytes(address_table)?),
        }
    }
}

#[derive(Debug)]
pub struct StatementLine {
    label: Option<String>,
    statement: Statement,
}

impl StatementLine {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens {
            [] => Err(Box::new(Error::TokensEmpty)),
            [Token::Label(label), tokens @ ..] => Ok(Self {
                label: Some(label.clone()),
                statement: Statement::from_tokens(tokens)?,
            }),
            [tokens @ ..] => Ok(Self {
                label: None,
                statement: Statement::from_tokens(tokens)?,
            }),
        }
    }

    pub fn byte_size(&self) -> usize {
        self.statement.byte_size()
    }

    pub fn as_bytes(
        &self,
        address_table: &AddressTable,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.statement.as_bytes(address_table)
    }
}

pub fn assemble(lines: Vec<Vec<Token>>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut statement_lines = vec![];
    let mut byte_code = vec![];
    let mut address = 0;

    let mut address_table = AddressTable::new();

    for line in lines {
        let statement_line = StatementLine::from_tokens(&line)?;

        if let Some(ref label) = statement_line.label {
            address_table.insert(label.clone(), Pep8Word::new(address as u16));
        }

        address += statement_line.byte_size();

        statement_lines.push(statement_line);
    }

    for statement_line in statement_lines {
        let bytes = statement_line.as_bytes(&address_table)?;

        byte_code.extend(bytes);
    }

    Ok(byte_code)
}
