pub mod chunk;
pub mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::init();
    let index = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant(index), 123);
    chunk.write(OpCode::OpReturn, 123);
    chunk.end_line_parsing();
    chunk.disassemble("test chunk");
}
