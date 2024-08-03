use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(version, about, long_about = None, author)]
#[command(arg_required_else_help = true)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set { key: String, value: String },
    /// Get the string value of a given string key
    Get { key: String },
    /// Remove a given key
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Set { .. } => panic!("unimplemented"),
        Commands::Get { .. } => panic!("unimplemented"),
        Commands::Rm { .. } => panic!("unimplemented"),
    }
}
