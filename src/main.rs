use clap::Parser;
use std::process;

mod cli;
mod config;
mod error;
mod fs_handler;
mod image_handler;
mod processor;
mod transformer;

fn main() {
    let cli = cli::Cli::parse();

    let config = config::Config::new(cli).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    if let Err(e) = processor::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
