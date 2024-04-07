use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod parsers;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        /// Enter directory where to initilize svcs
        #[arg(value_parser = parsers::is_directory)]
        directory: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { directory }) => commands::init(&directory),
        None => (),
    }
}
