use crate::chunk::{Chunk, OpCode};
use log::{log_enabled, Level};
use std::cell::Cell;

pub struct VM<'a> {
    chunk: &'a Chunk,
    // ip stands for 'instruction pointer' and stores a pointer to the current `OpCode`. We want to
    // wrap it in a `Cell` because we need to be able to mutate it, but we don't want to deal with
    // `&mut self` references, which would _also_ make `self.chunk` actually `&mut Chunk`, causing
    // a lot of problems when we try to call methods on it.
    ip: Cell<Option<usize>>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM<'_> {
    pub fn init(chunk: &Chunk) -> VM {
        VM {
            chunk,
            ip: Cell::new(None),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.ip = Cell::new(Some(0));
        self.run()
    }

    fn run(&self) -> InterpretResult {
        while let Some((opcode, index)) = self.read_byte() {
            if log_enabled!(Level::Debug) {
                let opcode = opcode.clone();
                self.chunk.disassemble_instruction(index, opcode)
            }

            match opcode {
                OpCode::OpReturn => break,
                OpCode::OpConstant(index) => {
                    let index = *index;
                    let value = self.chunk.constants[index];
                    println!("{}", value);
                }
            };
        }
        InterpretResult::Ok
    }

    // private, VM-use only functions
    fn read_byte(&self) -> Option<(&OpCode, usize)> {
        self.ip.take().and_then(|index| {
            self.ip.set(Some(index + 1));
            self.chunk.code.get(index).map(|opcode| (opcode, index))
        })
    }
}
