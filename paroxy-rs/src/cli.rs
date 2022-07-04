use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a source string or file
    Run {
        /// Program string or file.
        #[clap(value_parser)]
        source: String,

        /// The source is a file.
        #[clap(short, long, action)]
        file: bool,

        /// The source is brainfuck code.
        #[clap(short, long, action)]
        brainfuck: bool,

        /// The source is compiled binary data.
        #[clap(short, long, action)]
        compiled: bool,
    },

    /// Compile given program into binary bundle
    Compile {
        /// Program string or file.
        #[clap(value_parser)]
        source: String,

        /// The source is a file.
        #[clap(short, long, action)]
        file: bool,

        /// The source is brainfuck code.
        #[clap(short, long, action)]
        brainfuck: bool,

        /// The output path
        #[clap(value_parser)]
        out: Option<PathBuf>,
    },
}
