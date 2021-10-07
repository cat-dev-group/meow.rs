use clap::Clap;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Clap)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// the string to compile
    #[clap(name = "file")]
    file: String,
}

fn main() {
    // Parse clap args into readable input
    let args = Args::parse();

    // Setup the cache and read from the first file
    let filename = Path::new(&args.file);
    let file = File::open(filename).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file into a string");
}
