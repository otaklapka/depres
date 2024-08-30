mod depres;
mod k8s;

use clap::Parser;
use depres::depres;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "k8sdr")]
#[command(about = "K8s deployment resource aggregator", long_about = None)]
struct Cli {
    /// Paths to the files to process
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    let mut contents: Vec<String> = Vec::new();

    for file in args.files {
        contents.push(fs::read_to_string(file).expect("Failed to read the file"));
    }

    if let Err(error) = depres(contents) {
        println!("{:?}", error)
    }
}
