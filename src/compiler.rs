// use crate::token::TokenInfo;
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenInfo;
use crate::value::Value;
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
    panicking: bool,
    chunk: Chunk,
}

impl Parser<'_> {
    pub fn init(scanner: Scanner) -> Parser {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panicking: false,
            scanner,
            chunk: Chunk::init(),
        }
    }

    pub fn advance(mut self) {
        self.previous = self.current;

        loop {
            self.current = Some(self.scanner.scan_token());

            let token_info = self.scanner.scan_token();

            // we only report the first error we run into
            if token_info.token == Token::Error && !self.panicking {
                // some scanner error is being reported here
                self.had_error();
                error_at(&token_info, error_message_for(&token_info));
                self.current = Some(token_info);
            } else {
                self.current = Some(token_info);
                break;
            }
        }
    }

    pub fn consume(mut self, token_type: Token, message: String) {
        if self.current.as_ref().is_some_and(|t| t.token == token_type) {
            self.advance();
        } else {
            self.had_error();
            // TODO handle None case for `self.current` here
            self.current.map(|cur| error_at(&cur, message));
        }
    }

    fn emit_byte(mut self, op_code: OpCode) {
        if let Some(prev) = self.previous {
            self.chunk.write(op_code, prev.line);
        } else {
            error!("expected to find a previously parsed token!");
        }
    }

    fn end_compilation(self) {
        self.emit_return();
    }

    fn emit_return(self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn emit_constant(mut self, value: Value) {
        let index = self.chunk.add_constant(value);
        self.emit_byte(OpCode::OpConstant(index));
    }

    fn had_error(&mut self) {
        self.had_error = true;
        self.panicking = true;
    }

    // parser itself, I'm not sure this can be easily ported to Rust
    // instead, lets read through the whole chapter, understand how the Pratt parser works,
    // and then implement our own.
    fn expression(self) {
        return self;
        
    }

    fn number(self, value: Value) {
        self.emit_constant(value);
    }

    fn grouping(self) {
        self.expression();
        self.consume(Token::RightParen, String::from("Expect ')' after expression."));
    }


    // fn unary(self) {
}

fn error_message_for(token_info: &TokenInfo) -> String {
    match token_info.token.clone() {
        Token::EOF => String::from(" at end"),
        Token::Identifier(identifier) => format!(" at '{}'", identifier),
        Token::String(str) => format!(" at '{}'", str),
        Token::Number(num) => format!(" at '{}'", num),
        _ => String::from(""),
    }
}

fn error_at(token_info: &TokenInfo, message: String) {
    error!("[line {}] Error{}", token_info.line, message);
}

enum Precedence {
  PrecNone,
  PrecAssignment,  // =
  PrecOr,          // or
  PrecAnd,         // and
  PrecEquality,    // == !=
  PrecComparison,  // < > <= >=
  PrecTerm,        // + -
  PrecFactor,      // * /
  PrecUnary,       // ! -
  PrecCall,        // . ()
  PrecPrimary
}

fn precedence_for_op(op: Token) -> Precedence {
}
