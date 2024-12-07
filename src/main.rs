mod task;
use clap::{command, Args, Parser, Subcommand};
use task::Task;

#[derive(Parser)]
#[command(name = "task-tracker")]
#[command(about = "a task tracking cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Add {
        name: String,
    },

    #[command(arg_required_else_help = true)]
    Update {
        id: u32,
        name: String,
    },

    #[command(arg_required_else_help = true)]
    Delete {
        id: u32,
    },

    List {
        status: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Add { name } => Task::add(&name),
        Commands::Update { id, name } => Task::update(id, &name),
        Commands::Delete { id } => Task::delete(id),
        Commands::List { status } => Task::list_tasks(status),
    }
}

fn set_something(key: u32, value: String) {
    todo!()
}