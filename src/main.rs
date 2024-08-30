mod depres;
mod k8s;

use clap::Parser;
use depres::read_deployment_resources;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "depres")]
#[command(about = "K8s deployment resources aggregator", long_about = None)]
struct Cli {
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    let mut contents: Vec<String> = Vec::new();

    for file in args.files {
        contents.push(fs::read_to_string(file).expect("Failed to read the file"));
    }

    if let Err(error) = read_deployment_resources(contents) {
        println!("{:?}", error);
    }
}
