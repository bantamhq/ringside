mod commands;
mod config;
mod error;
mod git;
mod lock;
mod source;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ringside")]
#[command(about = "Sync git repositories into a project directory")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new ringside.toml file
    Init {
        /// Root directory for synced sources (default: .agents)
        root: Option<String>,
    },
    /// Sync all configured repositories
    Sync,
    /// Add a new source to the config (creates config if needed)
    Add {
        /// Repository URL, GitHub shorthand (owner/repo), or GitHub URL with path
        url: String,
        /// Destination path within root
        #[arg(short, long)]
        dest: Option<String>,
    },
    /// Show status of synced sources (outdated, up-to-date, etc.)
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { root } => commands::init::run(root.as_deref()),
        Commands::Sync => commands::sync::run(),
        Commands::Add { url, dest } => commands::add::run(&url, dest.as_deref()),
        Commands::Status => commands::status::run(),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
