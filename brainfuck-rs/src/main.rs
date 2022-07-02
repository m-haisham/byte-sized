mod lib;

use std::{fs, process::exit};

use lib::BrainFuck;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let program = match args.len() {
        2 => args[1].to_owned(),
        3 => {
            if args[1] != "-f" {
                panic!("Expected file path '-f'");
            }

            fs::read_to_string(args[2].clone()).unwrap()
        }
        _ => {
            println!("usage: brainfuck-rs (<program> | [-f] <path>)");
            exit(2);
        }
    };

    let mut compiler = BrainFuck::new();
    compiler.compile(program.as_str());
}
