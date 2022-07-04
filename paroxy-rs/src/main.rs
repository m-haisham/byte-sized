use std::{fs, path::PathBuf};

use bincode::{DefaultOptions, Options};
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
            compiled,
        } => {
            if compiled && !file {
                panic!("use '--file' flag when running compiled chunk.");
            } else if compiled && brainfuck {
                panic!("'--brainfuck' not supported with '--compiled'");
            }

            match get_chunk(source, file, brainfuck, compiled) {
                Ok(chunk) => run(chunk),
                Err(error) => panic!("{error}"),
            }
        }
        cli::Commands::Compile {
            source,
            file,
            brainfuck,
            out,
        } => {
            if !file && out.is_none() {
                println!("'--out' must be used when using raw program code.");
                return;
            }

            let program = get_program(source.clone(), file);

            let chunk = match parse(program, brainfuck) {
                Ok(chunk) => chunk,
                Err(_) => return,
            };

            let bytes = DefaultOptions::new()
                .with_varint_encoding()
                .serialize(&chunk)
                .expect("Failed to serialize data");

            let file = match out {
                Some(path) => path,
                None => {
                    let source_file = PathBuf::from(source);
                    let parent = source_file.parent().unwrap();

                    let out_stem = source_file.file_stem().unwrap().to_string_lossy();
                    let out_name = format!("{out_stem}.pxb");

                    parent.join(out_name)
                }
            };

            fs::write(file, bytes).expect("Failed to write bytecode.");
        }
    }
}

fn get_chunk(
    source: String,
    file: bool,
    brainfuck: bool,
    compiled: bool,
) -> Result<Chunk, &'static str> {
    if compiled {
        let bytes = fs::read(source).expect("Unable to read file.");

        match Chunk::from_bytes(&bytes) {
            Ok(chunk) => Ok(chunk),
            Err(_) => Err("Failed to load chunk from binary data."),
        }
    } else {
        let program = get_program(source, file);
        parse(program, brainfuck)
    }
}

fn get_program(source: String, file: bool) -> String {
    if file {
        fs::read_to_string(source).expect("Unable to read file.")
    } else {
        source
    }
}

fn parse(program: String, brainfuck: bool) -> Result<Chunk, &'static str> {
    let mut chunk = Chunk::new();
    let success = if brainfuck {
        let scanner = BfScanner::new(program.as_str());
        BfParser::new(scanner, &mut chunk).compile()
    } else {
        let scanner = PrScanner::new(program.as_str());
        PrParser::new(scanner, &mut chunk).compile()
    };

    if success {
        Ok(chunk)
    } else {
        Err("Compilation failed")
    }
}

fn run(chunk: Chunk) {
    let mut vm = VM::new(chunk);
    vm.run();
}
