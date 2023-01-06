use crate::token::{Token, TokenInfo};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    line: Cell<i32>,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn init(source: &str) -> Scanner {
        Scanner {
            chars: source.chars().peekable(),
            line: Cell::new(0),
        }
    }

    pub fn scan_token(&mut self) -> TokenInfo {
        self.skip_whitespace();

        if self.is_at_end() {
            return self.create_token(Token::EOF);
        }

        if self.peek_satisfies(|c| c.is_digit(10)) {
            return self.number();
        }

        if let Some(c) = self.chars.next() {
            return match c {
                '(' => self.create_token(Token::LeftParen),
                ')' => self.create_token(Token::RightParen),
                '{' => self.create_token(Token::LeftBrace),
                '}' => self.create_token(Token::RightBrace),
                ';' => self.create_token(Token::Semicolon),
                ',' => self.create_token(Token::Comma),
                '.' => self.create_token(Token::Dot),
                '-' => self.create_token(Token::Minus),
                '+' => self.create_token(Token::Plus),
                '/' => self.create_token(Token::Slash),
                '*' => self.create_token(Token::Star),
                '!' => {
                    let has_double_equal = self.matches('=');
                    self.create_token(match has_double_equal {
                        true => Token::BangEqual,
                        false => Token::Bang,
                    })
                }
                '=' => {
                    let has_double_equal = self.matches('=');
                    self.create_token(match has_double_equal {
                        true => Token::EqualEqual,
                        false => Token::Equal,
                    })
                }
                '<' => {
                    let has_double_equal = self.matches('=');
                    self.create_token(match has_double_equal {
                        true => Token::LessEqual,
                        false => Token::Less,
                    })
                }
                '>' => {
                    let has_double_equal = self.matches('=');
                    self.create_token(match has_double_equal {
                        true => Token::GreaterEqual,
                        false => Token::Greater,
                    })
                }
                '"' => self.string(),
                _ => todo!(),
            };
        }

        self.create_token(Token::Error)
    }

    /// provide one character of consume-if-matching lookahead
    fn matches(&mut self, c: char) -> bool {
        if let Some(t) = self.peek() {
            if t == c {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) {
        self.chars.next();
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peek_eq(&mut self, c: char) -> bool {
        self.chars.peek().map_or(false, |p| *p == c)
    }

    /// Check that the next character matches a supplied predicate, without needing to pull it out
    /// or do an `unwrap`.
    fn peek_satisfies(&mut self, test: fn(char) -> bool) -> bool {
        self.chars.peek().map_or(false, |c| test(*c))
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\r' | 't' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line.set(self.line.get() + 1);
                }
                '/' => {
                    self.advance();
                    if self.matches('/') {
                        while let Some(c) = self.peek() {
                            if c != '\n' && !self.is_at_end() {
                                self.advance()
                            } else {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn number(&mut self) -> TokenInfo {
        let mut digits = String::new();

        while self.peek_satisfies(|c| c.is_digit(10)) {
            digits.push(self.chars.next().unwrap());
        }

        if self.peek_eq('.') {
            digits.push(self.chars.next().unwrap());
        }

        while self.peek_satisfies(|c| c.is_digit(10)) {
            digits.push(self.chars.next().unwrap());
        }

        let num: f64 = digits.parse().unwrap();

        self.create_token(Token::Number(num))
    }

    fn string(&mut self) -> TokenInfo {
        let mut parsed_string = String::new();

        while let Some(c) = self.peek() {
            if c == '"' || self.is_at_end() {
                break;
            }
            if c == '\n' {
                self.line.set(self.line.get() + 1);
            }
            parsed_string.push(c);
            self.advance();
        }

        if self.is_at_end() {
            return self.create_token(Token::Error);
        }

        self.advance();
        self.create_token(Token::String(parsed_string))
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    fn create_token(&self, token: Token) -> TokenInfo {
        TokenInfo {
            token,
            line: self.line.get(),
        }
    }
}
