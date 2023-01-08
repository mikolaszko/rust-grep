#![allow(unused)]

use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Search for a pattern in a file and siplay the lines that contain it
#[derive(Parser)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
impl fmt::Display for Cli {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self)
    }
}
fn main() -> std::io::Result<()>{
    let args = Cli::parse();
    // println!("{0} {1}", args.pattern, args.path.display())
    let file = File::open(args.path)?;
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    for line in reader.lines() {
        match_pattern(line?, &args.pattern)
    }
    Ok(())
}

fn match_pattern(line: String, pattern: &String) {
    match &line {
        s if s.contains(pattern) => println!("{}", line),
        _ => print!(""),
    }
}
