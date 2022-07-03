use std::{env, fs};

use brainfuck::{parser::BfParser, scanner::BfScanner};
use chunk::Chunk;
use clap::Parser;
use paroxy::{parser::PrParser, scanner::PrScanner};
use vm::VM;

mod brainfuck;
mod chunk;
mod opcode;

mod cli;
mod debug;
mod paroxy;
mod vm;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Run {
            source,
            file,
            brainfuck,
        } => {
            let program = if file {
                fs::read_to_string(source).expect("Unable to read file.")
            } else {
                source
            };

            let chunk = parse(program, brainfuck);

            run(chunk);
        }
    }
}

fn parse(program: String, brainfuck: bool) -> Chunk {
    let mut chunk = Chunk::new();
    if brainfuck {
        let scanner = BfScanner::new(program.as_str());
        BfParser::new(scanner, &mut chunk).compile();
    } else {
        let scanner = PrScanner::new(program.as_str());
        PrParser::new(scanner, &mut chunk).compile();
    }

    chunk
}

fn run(chunk: Chunk) {
    let mut vm = VM::new(chunk);
    vm.run();
}
