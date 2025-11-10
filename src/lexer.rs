use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfLine,
    InvalidHexValue,
    InvalidEscapedChar,
    InvalidChar,
    InvalidNumber,
    InvalidString,
    InvalidDotCommand,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfLine => write!(f, "unexpected end of line"),
            Self::InvalidHexValue => write!(f, "invalid hex value"),
            Self::InvalidEscapedChar => write!(f, "invalid escaped character"),
            Self::InvalidChar => write!(f, "invalid character"),
            Self::InvalidNumber => write!(f, "invalid number"),
            Self::InvalidString => write!(f, "invalid string"),
            Self::InvalidDotCommand => write!(f, "invalid dot command"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Char(char),
    String(String),
    Number(u16),
    Comma,
    DotCommand(String),
    Label(String),
    Identifier(String),
}

fn parse_escaped_char<I: Iterator<Item = char>>(chars: &mut I) -> Result<char, Error> {
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
                        None => return Err(Error::UnexpectedEndOfLine),
                        Some(c) => hex_str.push(c),
                    }
                }

                match u32::from_str_radix(&hex_str.as_str(), 16) {
                    Ok(value) => match std::char::from_u32(value) {
                        None => Err(Error::InvalidHexValue),
                        Some(c) => Ok(c),
                    },
                    Err(_) => Err(Error::InvalidHexValue),
                }
            }
            '\"' => Ok('\"'),
            '\'' => Ok('\''),
            '\\' => Ok('\\'),
            _ => Err(Error::InvalidEscapedChar),
        }
    } else {
        Err(Error::UnexpectedEndOfLine)
    }
}

pub fn parse_line(line: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ';' => break,
            ' ' | '\t' => continue,
            ',' => tokens.push(Token::Comma),
            '\'' => {
                let value = match chars.next() {
                    None => return Err(Error::InvalidChar),
                    Some(next_c) => match next_c {
                        '\'' => return Err(Error::InvalidChar),
                        '\\' => parse_escaped_char(&mut chars)?,
                        _ => next_c,
                    },
                };

                match chars.next() {
                    Some('\'') => tokens.push(Token::Char(value)),
                    _ => return Err(Error::InvalidChar),
                }
            }
            '"' => {
                let mut value = String::new();

                loop {
                    match chars.next() {
                        None => return Err(Error::InvalidString),
                        Some(next_c) => match next_c {
                            '\"' => break,
                            '\\' => value.push(parse_escaped_char(&mut chars)?),
                            _ => value.push(next_c),
                        },
                    }
                }

                tokens.push(Token::String(value));
            }
            '-' | '+' | '0'..='9' => {
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
                        '0'..='9' => {
                            value.push(chars.next().expect("peeked character should not be None"))
                        }
                        ' ' | '\t' | ',' => break,
                        _ => return Err(Error::InvalidNumber),
                    }
                }

                let number_value = if value.starts_with("0x") || value.starts_with("0X") {
                    u16::from_str_radix(&value[2..], 16).or(Err(Error::InvalidNumber))?
                } else {
                    value.parse().or(Err(Error::InvalidNumber))?
                };

                tokens.push(Token::Number(number_value));
            }
            'a'..='z' | 'A'..='Z' | '_' | ':' | '.' => {
                let mut value = String::new();

                value.push(c);

                while let Some(next_c) = chars.peek() {
                    match next_c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            value.push(chars.next().expect("peeked character should not be None"));
                        }
                        _ => break,
                    }
                }

                if let Some(':') = chars.peek() {
                    if value.starts_with('.') {
                        return Err(Error::InvalidDotCommand);
                    }

                    chars.next();
                    tokens.push(Token::Label(value));
                } else if value.starts_with('.') {
                    tokens.push(Token::DotCommand(value));
                } else {
                    tokens.push(Token::Identifier(value));
                }
            }
            _ => return Err(Error::InvalidChar),
        }
    }

    Ok(tokens)
}

pub fn parse_file<P>(file_path: P) -> Result<Vec<Vec<Token>>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut token_lines = vec![];

    for line in reader.lines() {
        let tokens = parse_line(&line?)?;

        if !tokens.is_empty() {
            token_lines.push(tokens);
        }
    }

    Ok(token_lines)
}
