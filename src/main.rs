use clap::Parser;

mod commands;
use commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            commands::list_todos();
        }
        Commands::Add { name, description } => {
            commands::add_todo(name, description);
        }
        Commands::Remove { name } => {
            commands::remove_todo(name);
        }
        Commands::Complete { name } => {
            commands::complete_todo(name);
        }
    }
}
