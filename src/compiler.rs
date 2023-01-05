use crate::chunk::Chunk;
use crate::scanner::Scanner;
use crate::token::Token;
use std::io::{stdout, Write};

pub fn compile(source: &str) -> anyhow::Result<Chunk> {
    let mut scanner = Scanner::init(source);
    let mut line = -1;

    loop {
        let token_info = scanner.scan_token();
        let mut stdout_lock = stdout().lock();

        if token_info.line != line {
            write!(stdout_lock, "{:04} ", token_info.line)?;
            line = token_info.line;
        } else {
            write!(stdout_lock, "   | ")?;
        }

        writeln!(stdout_lock, "{:?}", token_info.token)?;
        stdout_lock.flush()?;

        if token_info.token == Token::EOF {
            break;
        }
    }

    Ok(Chunk::init())
}
