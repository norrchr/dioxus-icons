use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
mod build;
mod update;

mod parser;
//mod templates;
mod writer;
mod utils;

fn main() {
    let cli = CliArguments::parse();

    let config = config::read_config(cli.config).expect("Failed to read config");

    match cli.command {
        Commands::Build(args) => build::build_command(config, args),
        Commands::Update(args) => update::command_update(config, args)
    }
}

#[derive(Parser)]
#[command(version)]
#[command(about = "A CLI tool for dioxus-icons")]
struct CliArguments {
    #[clap(short, long)]
    config: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(build::BuildArgs),
    Update(update::UpdateArgs)
}