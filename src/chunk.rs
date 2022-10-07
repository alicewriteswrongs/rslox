use std::fmt;

#[derive(Debug)]
pub enum OpCode {
    OpReturn
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpReturn => f.pad("OP_RETURN")
        }
    }
}


#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>
}

impl Chunk {
    pub fn init() -> Chunk {
        Chunk {
            code: vec![]
        }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for (i, opcode) in self.code.iter().enumerate() {
            println!("{:0>4} {}", i, opcode);
        }
    }
}
