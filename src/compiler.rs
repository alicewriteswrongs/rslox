// use crate::token::TokenInfo;
use crate::chunk::Chunk;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenInfo;
use log::{error, log_enabled, Level};

pub fn compile(source: &str) -> anyhow::Result<Chunk> {
    if log_enabled!(Level::Debug) {
        // do the debug scan
        // we do this with a separate throwaway scanner instance
        let mut scanner = Scanner::init(source);
        scanner.debug_scan();
    }

    let scanner = Scanner::init(source);

    let parser = Parser::init(scanner);
    parser.advance();

    Ok(Chunk::init())
}

pub struct Parser<'a> {
    current: Option<TokenInfo>,
    previous: Option<TokenInfo>,
    scanner: Scanner<'a>,
    had_error: bool,
}

impl Parser<'_> {
    pub fn init(scanner: Scanner) -> Parser {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            scanner,
        }
    }

    pub fn advance(mut self) {
        self.previous = self.current;

        loop {
            self.current = Some(self.scanner.scan_token());
            if self
                .current
                .clone()
                .is_some_and(|token| token.token != Token::Error)
            {
                break;
            }
            error_at(&self.current.unwrap());
            self.had_error = true;
        }
    }
}

fn error_at(token: &TokenInfo) {
    let message = match token.token.clone() {
        Token::EOF => String::from(" at end"),
        Token::Identifier(identifier) => format!(" at '{}'", identifier),
        Token::String(str) => format!(" at '{}'", str),
        Token::Number(num) => format!(" at '{}'", num),
        _ => String::from(""),
    };

    error!("[line {}] Error{}", token.line, message);
}
