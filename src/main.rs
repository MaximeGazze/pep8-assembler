use address::{Address, AddressTable};
use dotcommand::DotCommand;
use instruction::Instruction;
use lexer::{Token, TokenType};

mod address;
mod dotcommand;
mod instruction;
mod lexer;
mod register;
mod types;

enum Statement {
    Instruction(Instruction),
    DotCommand(DotCommand),
}

impl Statement {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        match tokens.first() {
            None => Err(Box::from("tokens is empty")),
            Some(token) => match token.kind {
                TokenType::Identifier => Ok(Self::Instruction(Instruction::from_tokens(tokens)?)),
                TokenType::DotCommand => Ok(Self::DotCommand(DotCommand::from_tokens(tokens)?)),
                _ => Err(Box::from("invalid token type")),
            },
        }
    }
}

struct StatementLine {
    label: Option<String>,
    statement: Statement,
}

impl StatementLine {
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, Box<dyn std::error::Error>> {
        // fetch a possible label from the token list
        let label = match tokens.first() {
            None => return Err(Box::from("tokens is empty")),
            Some(token) => match token.kind {
                TokenType::Label => Some(token.value.clone()),
                _ => None,
            },
        };

        // determine if the statement on this line is an instruction or a dot command
        let statement = Statement::from_tokens(if label.is_some() {
            tokens
                .get(1..)
                .ok_or::<Box<dyn std::error::Error>>(Box::from("unexpected end of tokens"))?
        } else {
            tokens
        })?;

        Ok(Self { statement, label })
    }
}

fn assemble(lines: Vec<Vec<Token>>) -> Vec<u8> {
    let code = vec![];
    let address = 0;

    // TODO create instruction vector
    let address_table = AddressTable::new();

    for line in lines {
        // TODO
    }

    code
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
         .END    
"#,
    );

    let r = lexer::parse_string(s).unwrap();

    for l in r {
        println!("{:?}", l);
    }

    let res = Address::from_tokens_short(&[
        Token {
            kind: TokenType::String,
            value: String::from("ab"),
        },
        Token {
            kind: TokenType::Comma,
            value: String::from(","),
        },
        Token {
            kind: TokenType::Identifier,
            value: String::from("i"),
        },
    ]);

    if res.is_ok() {
        println!("{:?}", res);
    } else {
        println!("{:?}", res);
    }
}
