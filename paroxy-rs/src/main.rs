use std::{env, fs};

use brainfuck::{parser::Parser, scanner::Scanner};
use chunk::Chunk;
use paroxy::{parser::PrParser, scanner::PrScanner};
use vm::VM;

mod brainfuck;
mod chunk;
mod opcode;

mod debug;
mod paroxy;
mod vm;

fn main() {
    let default = fs::read_to_string("scripts/main.px").unwrap();
    let source = env::args().nth(1).unwrap_or(default);

    let scanner = PrScanner::new(source.as_str());
    let mut chunk = Chunk::new();

    let mut parser = PrParser::new(scanner, &mut chunk);
    parser.compile();

    let mut vm = VM::new(chunk);
    vm.run();
}
