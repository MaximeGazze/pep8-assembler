use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum Token {
    Char(char),
    String(String),
    Number(u16),
    Comma,
    DotCommand(String),
    Label(String),
    Identifier(String),
}

struct Lexer {
    chars: Peekable<IntoIter<char>>,
    row_pos: usize,
    col_pos: usize,
}

impl Lexer {
    fn new(s: &str) -> Self {
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
                // '-' => tokens.push(Token::Minus),
                // '+' => tokens.push(Token::Plus),
                ',' => tokens.push(Token::Comma),
                '\'' => {
                    let value = match self.next_char() {
                        None => return Err(Box::from("invalid char")),
                        Some(next_c) => match next_c {
                            '\'' => return Err(Box::from("invalid char")),
                            '\\' => self.parse_escaped_char()?,
                            _ => next_c,
                        },
                    };

                    match self.next_char() {
                        Some('\'') => tokens.push(Token::Char(value)),
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
                                '\\' => value.push(self.parse_escaped_char()?),
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

                    let number_value = if value.starts_with("0x") || value.starts_with("0X") {
                        u16::from_str_radix(&value[2..], 16)
                            .or::<Box<dyn std::error::Error>>(Err(Box::from("invalid number")))?
                    } else {
                        value
                            .parse()
                            .or::<Box<dyn std::error::Error>>(Err(Box::from("invalid number")))?
                    };

                    tokens.push(Token::Number(number_value));
                }
                'a'..='z' | 'A'..='Z' | '_' | ':' | '.' => {
                    let mut value = String::new();

                    value.push(c);

                    while let Some(next_c) = self.peek_next_char() {
                        match next_c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                value.push(self.next_char().unwrap());
                            }
                            ':' => {
                                if value.starts_with('.') {
                                    return Err(Box::from("invalid dot command"));
                                }

                                self.next_char();
                                tokens.push(Token::Label(value));

                                break;
                            }
                            _ => {
                                if value.starts_with('.') {
                                    tokens.push(Token::DotCommand(value));
                                } else {
                                    tokens.push(Token::Identifier(value));
                                }

                                break;
                            }
                        }
                    }
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

pub fn parse_str(s: &str) -> Result<Vec<Vec<Token>>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(s);

    lexer.parse()
}
