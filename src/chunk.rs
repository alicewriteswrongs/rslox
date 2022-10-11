use std::fmt;

use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    // the usize here is the index
    OpConstant(usize),
    OpReturn,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpReturn => f.pad("OP_RETURN"),
            OpCode::OpConstant(value) => f.pad(&format!("OP_CONSTANT: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn init() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for (i, opcode) in self.code.iter().enumerate() {
            match opcode {
                OpCode::OpConstant(index) => {
                    println!(
                        "{:0>4} {} '{}'",
                        i,
                        OpCode::OpConstant(*index),
                        self.constants[*index]
                    );
                }
                OpCode::OpReturn => {
                    println!("{:0>4} {}", i, OpCode::OpReturn);
                }
            }
        }
    }
}
