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

            let token_info = self.scanner.scan_token();

            if token_info.token == Token::Error {
                // some scanner error is being reported here
                error_at(&token_info);
                self.had_error = true;
                self.current = Some(token_info);
            } else {
                self.current = Some(token_info);
                break;
            }
        }
    }
}

fn error_at(token_info: &TokenInfo) {
    let message = match token_info.token.clone() {
        Token::EOF => String::from(" at end"),
        Token::Identifier(identifier) => format!(" at '{}'", identifier),
        Token::String(str) => format!(" at '{}'", str),
        Token::Number(num) => format!(" at '{}'", num),
        _ => String::from(""),
    };

    error!("[line {}] Error{}", token_info.line, message);
}
