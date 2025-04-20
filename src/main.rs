use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum TokenType {
    Char,
    String,
    Number,
    Minus,
    Plus,
    Comma,
    DotCommand,
    Label,
    Identifier,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: String,
}

impl Token {
    fn new(kind: TokenType, value: String) -> Self {
        Token { kind, value }
    }
}

fn handle_escaped_char(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<char, &'static str> {
    if let Some(c) = chars.next() {
        match c {
            'b' => Ok('\x08'),
            'f' => Ok('\x0C'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'v' => Ok('\x0B'),
            'x' | 'X' => {
                let mut hex_str = String::new();

                for _ in 0..2 {
                    match chars.next() {
                        None => return Err("iterator is empty"),
                        Some(c) => hex_str.push(c),
                    }
                }

                match u32::from_str_radix(&hex_str.as_str(), 16) {
                    Ok(value) => match std::char::from_u32(value) {
                        None => Err("invalid hex value"),
                        Some(c) => Ok(c),
                    },
                    Err(_) => Err("invalid hex value"),
                }
            }
            '\"' => Ok('\"'),
            '\'' => Ok('\''),
            '\\' => Ok('\\'),
            _ => Err("invalid escaped char"),
        }
    } else {
        Err("iterator is empty")
    }
}

fn tokenize_line(line: String) -> Result<Vec<Token>, &'static str> {
    let mut tokens = vec![];
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ';' => break,
            ' ' | '\t' => continue,
            '-' => tokens.push(Token::new(TokenType::Minus, String::new())),
            '+' => tokens.push(Token::new(TokenType::Plus, String::new())),
            ',' => tokens.push(Token::new(TokenType::Comma, String::new())),
            '\'' => {
                let mut value = String::new();

                match chars.next() {
                    None => return Err("invalid char"),
                    Some(next_c) => match next_c {
                        '\'' => return Err("invalid character"),
                        '\\' => {
                            if let Ok(escaped_char) = handle_escaped_char(&mut chars) {
                                value.push(escaped_char);
                            } else {
                                return Err("invalid char");
                            }
                        }
                        _ => value.push(next_c),
                    },
                }

                match chars.next() {
                    Some('\'') => tokens.push(Token::new(TokenType::Char, value)),
                    _ => return Err("invalid char"),
                }
            }
            '"' => {
                let mut value = String::new();

                loop {
                    match chars.next() {
                        None => return Err("invalid string"),
                        Some(next_c) => match next_c {
                            '\"' => break,
                            '\\' => {
                                if let Ok(escaped_char) = handle_escaped_char(&mut chars) {
                                    value.push(escaped_char);
                                } else {
                                    return Err("invalid string");
                                }
                            }
                            _ => value.push(next_c),
                        },
                    }
                }

                tokens.push(Token::new(TokenType::String, value));
            }
            '0'..='9' => {
                let mut value = String::new();

                value.push(c);

                if c == '0' {
                    if let Some(next_c) = chars.peek() {
                        if *next_c == 'x' || *next_c == 'X' {
                            value.push(chars.next().unwrap());
                        }
                    }
                }

                while let Some(next_c) = chars.peek() {
                    match next_c {
                        '0'..='9' => value.push(chars.next().unwrap()),
                        ' ' | '\t' | ',' => break,
                        _ => return Err("invalid number"),
                    }
                }

                tokens.push(Token::new(TokenType::Number, value));
            }
            'a'..='z' | 'A'..='Z' | '_' | ':' | '.' => {
                let mut value = String::new();
                let mut token_type = TokenType::Identifier;

                if c == '.' {
                    token_type = TokenType::DotCommand;
                }

                value.push(c);

                while let Some(next_c) = chars.peek() {
                    match next_c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            value.push(chars.next().unwrap());
                        }
                        ':' => {
                            if token_type == TokenType::DotCommand {
                                return Err("invalid dot command");
                            }

                            chars.next();
                            token_type = TokenType::Label;

                            break;
                        }
                        _ => break,
                    }
                }

                tokens.push(Token::new(token_type, value));
            }
            _ => return Err("invalid char"),
        }
    }

    Ok(tokens)
}

fn token_assemble(tokens: Vec<Vec<Token>>) -> Result<Vec<u8>, &'static str> {}

/* go over and parse all instruction
 * store tags
 * store tag calls
 * go over and replace tags by their address
 * */

/* we then go over each line and add the memory size and offset
 * while saving the address of tags
 * we then create our object code from each line (map)
 * */

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let x1 = tokenize_expr(String::from("BR main ; comment"));
    let x2 = tokenize_expr(String::from("retVal: .EQUATE 10"));
    let x3 = tokenize_expr(String::from("retVal: LDA 10,i"));
    let x4 = tokenize_expr(String::from("retVal: LDA 'a',i"));
    let x5 = tokenize_expr(String::from("retVal: LDA '\\n',i"));
    let x6 = tokenize_expr(String::from("retVal: LDA '\\\'',i"));
    let x7 = tokenize_expr(String::from("retVal: LDA '\"',i"));
    let x8 = tokenize_expr(String::from(
        "ra2:     ADDSP   6,i         ;pop #k #n #retVal ",
    ));
    let x9 = tokenize_expr(String::from("retVal: LDA 10x,i"));
    let x10 = tokenize_expr(String::from("x: .ASCII \"ok\\x41\""));

    println!("x1 = {x1:?}");
    println!("x2 = {x2:?}");
    println!("x3 = {x3:?}");
    println!("x4 = {x4:?}");
    println!("x5 = {x5:?}");
    println!("x6 = {x6:?}");
    println!("x7 = {x7:?}");
    println!("x8 = {x8:?}");
    println!("x9 = {x9:?}");
    println!("x10 = {x10:?}");

    println!("test");
    println!("{}", x5.unwrap()[2].value);
    println!("/test");
}
