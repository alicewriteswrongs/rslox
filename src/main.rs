pub mod chunk;
pub mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::init();
    let index = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant(index));
    chunk.write(OpCode::OpReturn);
    chunk.disassemble("test chunk");
}
