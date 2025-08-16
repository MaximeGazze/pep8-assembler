use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum TokenType {
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

// TODO change token type to token and store value in the enum

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    fn new(kind: TokenType, value: String) -> Self {
        Token { kind, value }
    }
}

struct Lexer {
    chars: Peekable<IntoIter<char>>,
    row_pos: usize,
    col_pos: usize,
}

impl Lexer {
    fn new(s: String) -> Self {
        let chars: Vec<_> = s.chars().collect();

        Self {
            chars: chars.into_iter().peekable(),
            row_pos: 0,
            col_pos: 0,
        }
    }

    fn peek_next_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next_char(&mut self) -> Option<char> {
        self.col_pos += 1;
        self.chars.next()
    }

    fn parse_escaped_char(&mut self) -> Result<char, Box<dyn std::error::Error>> {
        if let Some(c) = self.next_char() {
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
                        match self.next_char() {
                            None => return Err(Box::from("iterator is empty")),
                            Some(c) => hex_str.push(c),
                        }
                    }

                    match u32::from_str_radix(&hex_str.as_str(), 16) {
                        Ok(value) => match std::char::from_u32(value) {
                            None => Err(Box::from("invalid hex value")),
                            Some(c) => Ok(c),
                        },
                        Err(_) => Err(Box::from("invalid hex value")),
                    }
                }
                '\"' => Ok('\"'),
                '\'' => Ok('\''),
                '\\' => Ok('\\'),
                _ => Err(Box::from("invalid escaped char")),
            }
        } else {
            Err(Box::from("iterator is empty"))
        }
    }

    fn skip_to_end_of_line(&mut self) {
        while let Some(c) = self.peek_next_char() {
            if *c == '\n' {
                break;
            }

            self.next_char();
        }
    }

    fn parse(&mut self) -> Result<Vec<Vec<Token>>, Box<dyn std::error::Error>> {
        let mut lines = vec![];
        let mut tokens = vec![];

        while let Some(c) = self.next_char() {
            match c {
                ';' => self.skip_to_end_of_line(),
                '\n' => {
                    if !tokens.is_empty() {
                        lines.push(tokens);
                        tokens = vec![];
                    }

                    self.row_pos += 1;
                    self.col_pos = 0;
                }
                ' ' | '\t' => continue,
                '-' => tokens.push(Token::new(TokenType::Minus, String::new())),
                '+' => tokens.push(Token::new(TokenType::Plus, String::new())),
                ',' => tokens.push(Token::new(TokenType::Comma, String::new())),
                '\'' => {
                    let mut value = String::new();

                    match self.next_char() {
                        None => return Err("invalid char".into()),
                        Some(next_c) => match next_c {
                            '\'' => return Err(Box::from("invalid character")),
                            '\\' => {
                                if let Ok(escaped_char) = self.parse_escaped_char() {
                                    value.push(escaped_char);
                                } else {
                                    return Err(Box::from("invalid char"));
                                }
                            }
                            _ => value.push(next_c),
                        },
                    }

                    match self.next_char() {
                        Some('\'') => tokens.push(Token::new(TokenType::Char, value)),
                        _ => return Err(Box::from("invalid char")),
                    }
                }
                '"' => {
                    let mut value = String::new();

                    loop {
                        match self.next_char() {
                            None => return Err(Box::from("invalid string")),
                            Some(next_c) => match next_c {
                                '\"' => break,
                                '\\' => {
                                    if let Ok(escaped_char) = self.parse_escaped_char() {
                                        value.push(escaped_char);
                                    } else {
                                        return Err(Box::from("invalid string"));
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
                        if let Some(next_c) = self.peek_next_char() {
                            if *next_c == 'x' || *next_c == 'X' {
                                value.push(self.next_char().unwrap());
                            }
                        }
                    }

                    while let Some(next_c) = self.peek_next_char() {
                        match next_c {
                            '0'..='9' => value.push(self.next_char().unwrap()),
                            ' ' | '\t' | ',' => break,
                            _ => return Err(Box::from("invalid number")),
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

                    while let Some(next_c) = self.peek_next_char() {
                        match next_c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                value.push(self.next_char().unwrap());
                            }
                            ':' => {
                                if token_type == TokenType::DotCommand {
                                    return Err(Box::from("invalid dot command"));
                                }

                                self.next_char();
                                token_type = TokenType::Label;

                                break;
                            }
                            _ => break,
                        }
                    }

                    tokens.push(Token::new(token_type, value));
                }
                _ => return Err(Box::from("invalid char")),
            }
        }

        if !tokens.is_empty() {
            lines.push(tokens);
        }

        Ok(lines)
    }
}

pub fn parse_string(s: String) -> Result<Vec<Vec<Token>>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(s);

    lexer.parse()
}
