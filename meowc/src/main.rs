//! The `meowc` compiler
//!
//! The compiler as a whole is separated into the following phases (in order):
//!
//! lexing -> parsing -> other compiler stuff -> codegen
//!
//! Each phase corresponds to a source folder with roughly the same name, and are
//! labeled through the steps in this file.

mod errors;
mod lexer;

use ansi_term::Colour::Red;
use clap::Clap;
use lexer::Lexer;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    time::Instant,
};

#[derive(Clap)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// the string to compile
    #[clap(name = "file")]
    file: String,

    /// print out lexer output
    #[clap(long)]
    lex: bool,
}

fn main() {
    // Parse clap args into readable input
    let args = Args::parse();

    // Setup ansi_term on windows
    #[cfg(target_os = "windows")]
    let enabled = ansi_term::enable_ansi_support();

    // Setup the cache and read from the first file
    let filename = Path::new(&args.file);
    let file = File::open(filename).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file into a string");

    // Phase 1: Lexing
    let start = Instant::now();
    let tokens = Lexer::new(filename, &contents).collect::<Vec<_>>();
    let duration = start.elapsed().as_secs_f64();

    if args.lex {
        println!("{}", Red.paint(format!("Lexed in {:.2}s", duration)));
        tokens.iter().for_each(|(token, location)| {
            println!(
                "{} {:?} {} {}",
                Red.paint("|"),
                token,
                Red.paint("at"),
                location
            )
        });
    }
}
