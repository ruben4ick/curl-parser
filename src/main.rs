use clap::{Parser, Subcommand};
use curl_parser::parse;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[command(
    name = "curl_parser",
    about = "Parse curl commands into a structured model"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a file containing a single curl command and print the parsed model
    Parse { file: PathBuf },
    /// Show project credits
    Credits,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file } => {
            let content = match fs::read_to_string(file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to read: {}", e);
                    exit(1);
                }
            };
            match parse(&content) {
                Ok(req) => println!("{}", req),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(2);
                }
            }
        }
        Commands::Credits => {
            println!("curl_parser version: 0.1.0");
            println!("Author: ao.ruban@ukma.edu.ua");
            println!(
                "Description: A parser that takes a curl command as plain text input, parses it, and extracts structured information from it."
            );
        }
    }
}
