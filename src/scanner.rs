use crate::token::{check_keyword, Token, TokenInfo};
use log::{debug, log_enabled, Level};
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

        if self.peek_satisfies(|c| c.is_alphabetic()) {
            return self.identifier();
        }

        if self.peek_satisfies(|c| c.is_ascii_digit()) {
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
                '/' => {
                    if self.peek_eq('/') {
                        // consume that "/"
                        self.advance();
                        // when we get "//" that's the start of a comment!
                        return self.comment();
                    } else {
                        return self.create_token(Token::Slash);
                    }
                }
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

    /// Run a 'debug scan' which:
    /// 1. runs only in debug mode
    /// 2. consumes all tokens and prints them to the console
    pub fn debug_scan(&mut self) {
        if log_enabled!(Level::Debug) {
            let mut line = -1;
            debug!("scanning tokens...");
            loop {
                let token_info = self.scan_token();

                if token_info.line != line {
                    debug!("{:04} {:?}", token_info.line, token_info.token);
                    line = token_info.line;
                } else {
                    debug!("   | {:?}", token_info.token);
                }

                if token_info.token == Token::EOF {
                    break;
                }
            }
        }
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

    // consume one character
    fn advance(&mut self) {
        self.chars.next();
    }

    // one character of lookahead
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
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line.set(self.line.get() + 1);
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn identifier(&mut self) -> TokenInfo {
        let mut parsed_identifier = String::new();

        while self.peek_satisfies(|c| c.is_alphabetic() || c.is_ascii_digit()) {
            parsed_identifier.push(self.chars.next().unwrap());
        }

        self.create_token(
            check_keyword(&parsed_identifier).unwrap_or(Token::Identifier(parsed_identifier)),
        )
    }

    fn number(&mut self) -> TokenInfo {
        let mut digits = String::new();

        while self.peek_satisfies(|c| c.is_ascii_digit()) {
            digits.push(self.chars.next().unwrap());
        }

        if self.peek_eq('.') {
            digits.push(self.chars.next().unwrap());
        }

        while self.peek_satisfies(|c| c.is_ascii_digit()) {
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

    fn comment(&mut self) -> TokenInfo {
        let mut comment_string = String::new();

        while let Some(c) = self.peek() {
            if c != '\n' && !self.is_at_end() {
                self.advance();
                comment_string.push(c);
            } else {
                break;
            }
        }
        self.create_token(Token::Comment(comment_string))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn test_string_is_token(test_string: &str, token: Token) {
        let mut scanner = Scanner::init(&test_string);
        let token_info = scanner.scan_token();

        if token_info.token != token {
            println!("didnt match: {:?} and {:?}", token_info, token);
        }
        assert!(token_info.token == token);
    }

    fn test_tokens(test_string: &str, tokens: Vec<Token>) {
        let mut found_tokens: Vec<Token> = vec![];

        let mut scanner = Scanner::init(&test_string);

        loop {
            let token_info = scanner.scan_token();
            found_tokens.push(token_info.token.clone());

            if token_info.token == Token::EOF {
                break;
            }
        }
        if found_tokens != tokens {
            println!("didnt match: {:?} and {:?}", found_tokens, tokens);
        }
        assert!(found_tokens == tokens)
    }

    #[test]
    fn test_multiple_booleans() {
        let test_string = "true false";
        let mut scanner = Scanner::init(&test_string);
        assert!(scanner.scan_token().token == Token::True);
        assert!(scanner.scan_token().token == Token::False);
    }

    #[test]
    fn test_keywords() {
        test_string_is_token("and", Token::And);
        test_string_is_token("class", Token::Class);
        test_string_is_token("else", Token::Else);
        test_string_is_token("false", Token::False);
        test_string_is_token("for", Token::For);
        test_string_is_token("fun", Token::Fun);
        test_string_is_token("if", Token::If);
        test_string_is_token("nil", Token::Nil);
        test_string_is_token("or", Token::Or);
        test_string_is_token("print", Token::Print);
        test_string_is_token("return", Token::Return);
        test_string_is_token("super", Token::Super);
        test_string_is_token("this", Token::This);
        test_string_is_token("true", Token::True);
        test_string_is_token("var", Token::Var);
        test_string_is_token("while", Token::While);
    }

    #[test]
    fn simple_identifier() {
        test_string_is_token("foobar", Token::Identifier(String::from("foobar")));
    }

    #[test]
    fn simple_string() {
        test_string_is_token("\"test\"", Token::String(String::from("test")));
    }

    #[test]
    fn basic_numbers() {
        test_string_is_token("0", Token::Number(0.0));
        test_string_is_token("1", Token::Number(1.0));
        test_string_is_token("2", Token::Number(2.0));
        test_string_is_token("3", Token::Number(3.0));
        test_string_is_token("4", Token::Number(4.0));
        test_string_is_token("5", Token::Number(5.0));
        test_string_is_token("6", Token::Number(6.0));
        test_string_is_token("7", Token::Number(7.0));
        test_string_is_token("8", Token::Number(8.0));
        test_string_is_token("9", Token::Number(9.0));
    }

    #[test]
    fn punctuation() {
        test_string_is_token("(", Token::LeftParen);
        test_string_is_token(")", Token::RightParen);
        test_string_is_token("{", Token::LeftBrace);
        test_string_is_token("}", Token::RightBrace);
        test_string_is_token(",", Token::Comma);
        test_string_is_token(".", Token::Dot);
        test_string_is_token("-", Token::Minus);
        test_string_is_token("+", Token::Plus);
        test_string_is_token(";", Token::Semicolon);
        test_string_is_token("/", Token::Slash);
        test_string_is_token("*", Token::Star);
        test_string_is_token("!", Token::Bang);
        test_string_is_token("!=", Token::BangEqual);
        test_string_is_token("=", Token::Equal);
        test_string_is_token("==", Token::EqualEqual);
        test_string_is_token(">", Token::Greater);
        test_string_is_token(">=", Token::GreaterEqual);
        test_string_is_token("<", Token::Less);
        test_string_is_token("<=", Token::LessEqual);
    }

    #[test]
    fn test_whitespace() {
        test_string_is_token("  /    \t", Token::Slash);

        test_tokens(
            "\t    / +   \n/",
            vec![Token::Slash, Token::Plus, Token::Slash, Token::EOF],
        )
    }

    #[test]
    fn test_random_mixes() {
        test_tokens(
            "1 + 2;",
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
                Token::Semicolon,
                Token::EOF,
            ],
        );

        test_tokens(
            "1 + \"just a string\";",
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::String(String::from("just a string")),
                Token::Semicolon,
                Token::EOF,
            ],
        );
    }

    #[test]
    fn test_comment() {
        test_tokens(
            "1 + 2; // add those numbers!",
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
                Token::Semicolon,
                Token::Comment(String::from(" add those numbers!")),
                Token::EOF,
            ],
        );
    }

    #[test]
    fn test_comment_with_newline() {
        test_tokens(
            "1 + 2; // add those numbers!\nfoo",
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
                Token::Semicolon,
                Token::Comment(String::from(" add those numbers!")),
                Token::Identifier(String::from("foo")),
                Token::EOF,
            ],
        );
    }
}
