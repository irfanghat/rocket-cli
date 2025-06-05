use clap::Parser;
use colored::*;
use commands::Command;
use is_terminal::*;

mod commands;
mod templates;

#[derive(Parser)]
#[command(name = "rocket-cli")]
#[command(
    version,
    about = "A command-line interface (CLI) for developing, building and running Rocket Web applications."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => commands::handle_command(cmd),
        None => print_welcome(),
    }
}

fn print_welcome() {
    let is_tty = std::io::stdout().is_terminal();

    let rocket_art = r#"
         .
        / \
       / _ \        🚀  Rocket CLI
      | (_) |       ==========================================
      |  _  |       A command-line interface (CLI) for developing, 
     /| (_) |\      building and running Rocket Web applications.
    | |     | |     Write fast, type-safe, secure web apps with
    |_|     |_|     incredible usability, productivity & performance.
    "#;

    if is_tty {
        println!("{}", rocket_art.bright_red());
        println!("{}", "Usage Examples:".bold());
        println!(
            "  {}      {}",
            "rocket new my-api".cyan(),
            "Scaffold a new Rocket project"
        );
        println!(
            "  {}  {}",
            "rocket new --list".cyan(),
            "List available templates"
        );
        println!(
            "  {}         {}",
            "rocket run".cyan(),
            "Run your Rocket application"
        );

        println!();
        println!("{}", "Docs & Links:".bold());
        println!(
            "  {}     {}",
            "📘 Docs:".yellow(),
            "https://rocket.rs".underline()
        );
        println!(
            "  {}  {}",
            "💻 GitHub:".yellow(),
            "https://github.com/irfanghat/rocket-cli".underline()
        );
    } else {
        println!("{}", rocket_art);
        println!("Usage Examples:");
        println!("  rocket new my-api      Scaffold a new Rocket project");
        println!("  rocket new --list      List available templates");
        println!("  rocket run             Run your Rocket application");

        println!();
        println!("Docs & Links:");
        println!("  📘 Docs:     https://rocket.rs");
        println!("  💻 GitHub:   https://github.com/irfanghat/rocket-cli");
    }
}
