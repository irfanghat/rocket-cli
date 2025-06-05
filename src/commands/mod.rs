pub mod add;
pub mod build;
pub mod new;
pub mod run;

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new Rocket project
    New(NewArgs),

    /// Run the Rocket application
    Run,
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Project name
    pub name: Option<String>,

    /// Initialize a git repository
    #[arg(long, help = "Initialize a git repository")]
    pub git: bool,

    /// Template name
    #[arg(
        long,
        default_value = "minimal",
        help = "Choose template: minimal, mongodb, postgres, mysql, mssql, sqlite"
    )]
    pub template: String,

    /// List all available templates
    #[arg(long, help = "List available templates")]
    pub list: bool,
}

pub fn handle_command(cmd: Command) {
    match cmd {
        Command::New(args) => new::handle(args),
        Command::Run => run::execute(),
    }
}
