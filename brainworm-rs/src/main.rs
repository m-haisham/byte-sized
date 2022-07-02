use brainfuck::{parser::Parser, scanner::Scanner};
use chunk::Chunk;
use opcode::OpCode;

mod brainfuck;
mod chunk;
mod opcode;

mod debug;

fn main() {
    let scanner = Scanner::new(",[->+<]");
    let mut chunk = Chunk::new();

    let mut parser = Parser::new(scanner, &mut chunk);
    parser.compile();
}
