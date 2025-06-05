extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::{Parser, Subcommand};

mod commands;
mod templates;

#[derive(Parser)]
#[command(name = "rocket")]
#[command(
    about = "A command-line interface (CLI) for developing, building and running Rocket Web applications."
)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

fn main() {
    let cli = Cli::parse();
    commands::handle_command(cli.command);
}
