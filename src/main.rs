use anyhow::{anyhow, Result};
// use log::{log_enabled, Level};
use std::env::args;
use std::fs;
use std::io::{stdin, stdout, BufRead, Write};

pub mod chunk;
pub mod compiler;
pub mod scanner;
pub mod token;
pub mod value;
pub mod vm;

use vm::VM;

fn repl() -> Result<()> {
    let vm = VM::init();

    loop {
        // acquire lock on stdout, print our little prompt
        let mut stdout_lock = stdout().lock();
        write!(stdout_lock, "> ")?;
        stdout_lock.flush()?;

        let mut stdin_handle = stdin().lock();
        let mut buffer = String::new();
        stdin_handle.read_line(&mut buffer)?;
        vm.interpret(&buffer);
    }
}

fn run_file(filename: &String) -> Result<()> {
    let source = fs::read_to_string(filename)?;
    let vm = VM::init();
    vm.interpret(&source);
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let arguments: Vec<String> = args().collect();

    match arguments.len() {
        1 => repl(),
        2 => run_file(&arguments[1]),
        _ => {
            // log::error!("Usage: rslox [path]\n");
            Err(anyhow!("Usage: rslox [path]\n"))
        }
    }
}
