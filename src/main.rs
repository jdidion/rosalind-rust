use rosalind::{Input, dna, rna};

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, default_value = "1")]
    problem: usize,

    #[clap(short, long)]
    input: Option<String>,

    #[clap(short, long)]
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    let input = if let Some(input) = args.input {
        Input::new(&input)
    } else if let Some(path) = args.file {
        Input::from_file(&path)
    } else {
        panic!("No input provided.");
    };

    let answer = match args.problem {
        1 => dna::counting::run(input),
        2 => rna::transcribe::run(input),
        3 => dna::complement::run(input),
        4 => dna::hamming::run(input),
        _ => panic!("Invalid problem or arguments."),
    };

    answer.print();
}
