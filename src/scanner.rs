use crate::token::Token;
use crate::token::TokenType;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

struct Scanner {
    // start: &'a char,
    // current: &'a char,
    line: Cell<i32>,
    chars: RefCell<VecDeque<char>>,
    state: Cell<ScannerState>,
}

enum ScannerState {
    Ready,
    ParsingToken(String),
    ParsingWhitespace,
}

impl Scanner {
    pub fn init(source: &String) -> Scanner {
        let chars: VecDeque<char> = source.chars().collect();

        Scanner {
            chars: RefCell::new(chars),
            line: Cell::new(0),
            state: Cell::new(ScannerState::Ready),
        }
    }

    pub fn scan_token(&self) -> Token {
        if self.is_at_end() {
            return self.create_token(TokenType::EOF);
        }

        if let Some(c) = self.chars.borrow_mut().pop_front() {
            return match c {
                '(' => self.create_token(TokenType::LeftParen),
                ')' => self.create_token(TokenType::RightParen),
                '{' => self.create_token(TokenType::LeftBrace),
                '}' => self.create_token(TokenType::RightBrace),
                ';' => self.create_token(TokenType::Semicolon),
                ',' => self.create_token(TokenType::Comma),
                '.' => self.create_token(TokenType::Dot),
                '-' => self.create_token(TokenType::Minus),
                '+' => self.create_token(TokenType::Plus),
                '/' => self.create_token(TokenType::Slash),
                '*' => self.create_token(TokenType::Star),
                '!' => self.create_token(match self.matches('=') {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                }),
                _ => todo!(),
            };
        }

        self.create_token(TokenType::Error)
    }

    /// provide one character of consume-if-matching lookahead
    fn matches(&self, c: char) -> bool {
        if let Some(t) = self.chars.borrow().get(0) {
            if *t == c {
                self.chars.borrow_mut().pop_front();
                return true;
            }
        }
        return false;
    }

    fn is_at_end(&self) -> bool {
        self.chars.borrow().len() == 0
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        // we're producing a token, so back to the 'ready' state
        self.state.set(ScannerState::Ready);
        Token {
            token_type,
            line: self.line.get(),
        }
    }
}
