use address::{Address, AddressTable};
use dotcommand::DotCommand;
use instruction::Instruction;
use lexer::Token;
use types::Pep8Word;

mod address;
mod dotcommand;
mod instruction;
mod lexer;
mod register;
mod types;

#[derive(Debug)]
enum Statement {
    Instruction(Instruction),
    DotCommand(DotCommand),
}

impl Statement {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens.first() {
            None => Err(Box::from("tokens is empty")),
            Some(token) => match token {
                Token::Identifier(_) => Ok(Self::Instruction(Instruction::from_tokens(tokens)?)),
                Token::DotCommand(_) => Ok(Self::DotCommand(DotCommand::from_tokens(tokens)?)),
                _ => Err(Box::from("invalid token type")),
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
            Self::DotCommand(dotcommand) => dotcommand.as_bytes(address_table),
        }
    }
}

#[derive(Debug)]
struct StatementLine {
    label: Option<String>,
    statement: Statement,
}

impl StatementLine {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens {
            [] => Err(Box::from("tokens is empty")),
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

fn assemble(lines: Vec<Vec<Token>>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

fn main() {
    let s = String::from(
        r#"
;File: exer0804.pep
;Computer Systems, Fourth edition
;Exercise 8.4
;
         BR      main        ;Branch around data
num:     .BLOCK  2           ;Global variable
main:    DECI    num,d       ;Input decimal value
         DECO    num,d       ;Output decimal value
         CHARO   '\n',i      
         STRO    msg,d       ;Output message
         STOP                
msg:     .ASCII  "That's all.\n\x00"
a:       .ADDRSS num
         .END    
"#,
    );

    let r = lexer::parse_str(&s).unwrap();

    for l in &r {
        println!("{:?}", l);
    }

    let byte_code = assemble(r).unwrap();

    println!("{:02X?}", byte_code);

    let res = Address::from_tokens_short(&[
        Token::String(String::from("ab")),
        Token::Comma,
        Token::Identifier(String::from("i")),
    ]);

    if res.is_ok() {
        println!("{:?}", res);
    } else {
        println!("{:?}", res);
    }
}
