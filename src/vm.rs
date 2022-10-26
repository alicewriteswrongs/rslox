use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use log::{log_enabled, Level};
use std::cell::{Cell, RefCell};

pub struct VM<'a> {
    chunk: &'a Chunk,
    // ip stands for 'instruction pointer' and stores a pointer to the current `OpCode`. We want to
    // wrap it in a `Cell` because we need to be able to mutate it, but we don't want to deal with
    // `&mut self` references, which would _also_ make `self.chunk` actually `&mut Chunk`, causing
    // a lot of problems when we try to call methods on it.
    ip: Cell<Option<usize>>,
    stack: RefCell<Vec<Value>>,
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
            stack: RefCell::new(vec![]),
        }
    }

    pub fn interpret(&self) -> InterpretResult {
        self.ip.set(Some(0));
        self.run()
    }

    fn run(&self) -> InterpretResult {
        while let Some((opcode, index)) = self.read_byte() {
            if log_enabled!(Level::Debug) {
                let opcode = opcode.clone();
                self.print_stack();
                self.chunk.disassemble_instruction(index, opcode);
            }

            match opcode {
                OpCode::OpReturn => {
                    let mut stack = self.stack.borrow_mut();
                    let value = stack.pop();

                    if let Some(val) = value {
                        println!("{}", val);
                    }
                }
                OpCode::OpConstant(index) => {
                    let index = *index;
                    let value = self.chunk.constants[index];
                    let mut stack = self.stack.borrow_mut();
                    stack.push(value);
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

    fn print_stack(&self) {
        println!("Stack: {:?}", self.stack.borrow());
    }
}
