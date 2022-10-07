pub mod chunk;
pub mod memory;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::init();
    chunk.write(OpCode::OpReturn);
    chunk.disassemble("test chunk");
}
