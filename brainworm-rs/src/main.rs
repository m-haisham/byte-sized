use std::env;

use brainfuck::{parser::Parser, scanner::Scanner};
use chunk::Chunk;
use vm::VM;

mod brainfuck;
mod chunk;
mod opcode;

mod debug;
mod vm;

fn main() {
    let default = "++>+<[->+<].".to_owned();
    let source = env::args().nth(1).unwrap_or(default);

    let scanner = Scanner::new(source.as_str());
    let mut chunk = Chunk::new();

    let mut parser = Parser::new(scanner, &mut chunk);
    parser.compile();

    let mut vm = VM::new(chunk);
    vm.run();
}
