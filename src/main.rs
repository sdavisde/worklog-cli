use clap::{Parser, Subcommand};

mod config;
#[path = "commands/daily_note.rs"]
mod daily_note;
#[path = "commands/note.rs"]
mod note;
#[path = "commands/task.rs"]
mod task;
mod utils;

use daily_note::open_daily_note;
use task::add_task;
use note::add_note;

use crate::daily_note::open_last_daily_note;

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
            open_daily_note(config).expect("Failed to open daily note");
        }
        Some(Commands::Last) => {
            open_last_daily_note(config).expect("Failed to open last daily note");
        }
        Some(Commands::Task { description }) => {
            if description.is_empty() {
                println!("Cannot add a task without a <description>");
            }
            add_task(description);
        }
        Some(Commands::Note { description }) => {
            if description.is_empty() {
                println!("Cannot add a note without a <description>");
            }
            add_note(description);
        }
    }

    // Continued program logic goes here...
}
