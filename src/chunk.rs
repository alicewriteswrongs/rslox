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
pub struct OpcodeLine {
    // the index within the associated chunk of the first instruction on the line
    start: usize,
    // the index within the associated chunk of the last instruction on the line
    end: usize,
    line_number: i32,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<OpcodeLine>,
    current_line: Option<i32>,
}

impl Chunk {
    pub fn init() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![],
            current_line: None,
        }
    }

    pub fn write(&mut self, byte: OpCode, line_number: i32) {
        self.code.push(byte);
        self.parse_line(line_number);
    }

    pub fn parse_line(&mut self, line_number: i32) {
        if let Some(current_line_number) = self.current_line {
            if current_line_number != line_number {
                self.current_line = Some(line_number);
                self.lines.push(OpcodeLine {
                    start: self.lines.last().map(|line| line.start).unwrap_or(0),
                    end: self.code.len() - 1,
                    line_number,
                })
            }
        } else {
            self.current_line = Some(line_number);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn get_line_number(&self, index: usize) -> Option<i32> {
        self.lines
            .iter()
            .find(|line| line.start <= index && index <= line.end)
            .map(|line| line.line_number)
    }

    pub fn end_line_parsing(&mut self) {
        if let Some(current_line) = self.current_line {
            self.lines.push(OpcodeLine {
                start: self.lines.last().map(|line| line.start).unwrap_or(0),
                end: self.code.len() - 1,
                line_number: current_line,
            });
            self.current_line = None;
        }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        println!("{:?}", self);

        for (i, opcode) in self.code.iter().enumerate() {
            match opcode {
                OpCode::OpConstant(index) => {
                    println!(
                        "{:0>4} {} {} '{}'",
                        i,
                        self.get_line_number(i).unwrap_or(-1),
                        OpCode::OpConstant(*index),
                        self.constants[*index]
                    );
                }
                OpCode::OpReturn => {
                    println!(
                        "{:0>4} {} {} ",
                        i,
                        self.get_line_number(i).unwrap_or(-1),
                        OpCode::OpReturn
                    );
                }
            }
        }
    }
}
