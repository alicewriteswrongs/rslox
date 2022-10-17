use log::{log_enabled, Level};

pub mod chunk;
pub mod value;
pub mod vm;

use chunk::{Chunk, OpCode};
use vm::VM;

fn main() {
    env_logger::init();

    let mut chunk = Chunk::init();

    let index = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant(index), 123);
    chunk.write(OpCode::OpReturn, 123);
    chunk.end_line_parsing();

    if log_enabled!(Level::Debug) {
        chunk.disassemble("test chunk");
    }

    let vm = VM::init(&chunk);
    vm.interpret();
}
