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

    let index = chunk.add_constant(3.4);
    chunk.write(OpCode::OpConstant(index), 123);

    chunk.write(OpCode::OpAdd, 123);

    let index = chunk.add_constant(5.6);
    chunk.write(OpCode::OpConstant(index), 123);

    chunk.write(OpCode::OpDivide, 123);

    chunk.write(OpCode::OpNegate, 123);
    chunk.write(OpCode::OpReturn, 123);
    chunk.end_line_parsing();

    if log_enabled!(Level::Debug) {
        chunk.disassemble("test chunk");
    }

    let vm = VM::init(&chunk);
    vm.interpret();
}
