pub mod add;
pub mod build;
pub mod new;
pub mod run;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    New {
        name: String,

        #[arg(long, help = "Initialize a git repository")]
        git: bool,

        #[arg(
            long,
            default_value = "minimal",
            help = "Choose template: minimal, api, etc."
        )]
        template: String,
    },
    Run,
}

pub fn handle_command(cmd: Command) {
    match cmd {
        Command::New {
            name,
            git,
            template,
        } => new::execute(name, git, template),
        Command::Run => run::execute(),
    }
}
