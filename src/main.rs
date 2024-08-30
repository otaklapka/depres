mod k8s;
mod depres;

use std::{fs, path::PathBuf};
use depres::depres;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(name = "k8sdr")]
#[command(about = "A tool to parse and sum resources from multiple k8s deployment files", long_about = None)]
struct Cli {
    /// Paths to the files to process
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    let mut contents: Vec<String> = Vec::new();

    for file in args.files {
        contents.push(
            fs::read_to_string(file)
                .expect("Failed to read the file")
        );
    }

    if let Err(error) = depres(contents) {
        println!("{:?}", error)
    }
}