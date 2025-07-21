use clap::{Parser, Subcommand};

mod config;
mod utils;
mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Open,
    Last,
    Task {
        description: String,
        // todo: would be cool to be able to do something like `wl task "Draft contract for Will" --project-planning`
        // #[arg(long)]
        // category: String,
    },
    Note {
        description: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let config = config::load_config().expect("Failed to parse config");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Open) | None => {
            commands::daily_note::open_daily_note(config).expect("Failed to open daily note");
        }
        Some(Commands::Last) => {
            commands::daily_note::open_last_daily_note(config).expect("Failed to open last daily note");
        }
        Some(Commands::Task { description }) => {
            if description.is_empty() {
                println!("Cannot add a task without a <description>");
            }
            commands::task::add_task(description);
        }
        Some(Commands::Note { description }) => {
            if description.is_empty() {
                println!("Cannot add a note without a <description>");
            }
            commands::note::add_note(description);
        }
    }

    // Continued program logic goes here...
}
