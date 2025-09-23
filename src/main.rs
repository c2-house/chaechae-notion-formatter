use clap::Parser;
use std::process;

mod cli;
mod config;
mod error;
mod fs_handler;
mod transformer;

fn main() {
    let cli = cli::Cli::parse();

    let config = config::Config::new(cli).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    let content = fs_handler::read_file(&config.source_file_path).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    let transformed_text = transformer::transform_text(&content);
    println!("{}", transformed_text);
}
