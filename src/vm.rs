use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::value::Value;
use log::{log_enabled, Level};
use std::cell::{Cell, RefCell};

pub struct VM {
    // ip stands for 'instruction pointer' and stores a pointer to the current `OpCode`. We want to
    // wrap it in a `Cell` because we need to be able to mutate it, but we don't want to deal with
    // `&mut self` references.
    ip: Cell<Option<usize>>,
    stack: RefCell<Vec<Value>>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_op{
    ($self:ident, $op:tt)=> {
        {
            let mut stack = $self.stack.borrow_mut();
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a $op b);
        }
    }
}

impl VM {
    pub fn init() -> VM {
        VM {
            ip: Cell::new(None),
            stack: RefCell::new(vec![]),
        }
    }

    pub fn interpret(&self, source: &String) -> InterpretResult {
        let chunk = compile(source);
        self.ip.set(Some(0));
        self.run(&chunk)
    }

    // private, VM-use only functions
    fn run(&self, chunk: &Chunk) -> InterpretResult {
        while let Some((opcode, index)) = self.read_byte(chunk) {
            if log_enabled!(Level::Debug) {
                self.print_stack();
                chunk.disassemble_instruction(index, opcode);
            }

            match opcode {
                OpCode::OpReturn => {
                    let mut stack = self.stack.borrow_mut();
                    let value = stack.pop();

                    if let Some(val) = value {
                        println!("{}", val);
                    }
                }
                OpCode::OpAdd => binary_op!(self, +),
                OpCode::OpSubtract => binary_op!(self, -),
                OpCode::OpMultiply => binary_op!(self, *),
                OpCode::OpDivide => binary_op!(self, /),
                OpCode::OpNegate => {
                    let mut stack = self.stack.borrow_mut();
                    if let Some(num) = stack.pop() {
                        stack.push(-num);
                    } else {
                        // TODO do something more sensible here
                        panic!(
                            "{}",
                            format!("I tried to negate something that wasn't there :/")
                        )
                    }
                }
                OpCode::OpConstant(index) => {
                    let index = *index;
                    let value = chunk.constants[index];
                    let mut stack = self.stack.borrow_mut();
                    stack.push(value);
                }
            };
        }
        InterpretResult::Ok
    }

    fn read_byte<'a>(&'a self, chunk: &'a Chunk) -> Option<(&OpCode, usize)> {
        self.ip.take().and_then(|index| {
            self.ip.set(Some(index + 1));
            chunk.code.get(index).map(|opcode| (opcode, index))
        })
    }

    fn print_stack(&self) {
        println!("Stack: {:?}", self.stack.borrow());
    }
}
