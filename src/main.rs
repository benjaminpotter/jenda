use clap::{Args, Parser, Subcommand};
use jenda::{Database, JendaError, Task, VecDatabase};
use tabled::Table;

#[derive(Parser)]
struct JendaCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task.
    Add(AddOptions),

    /// Mark a task as complete.
    Complete,

    /// Display a list of all tasks.
    List(ListOptions),

    /// Display info for a single task.
    Info,

    /// Manage configuration options.
    Config,
}

fn main() {
    let cli = JendaCli::parse();
    let mut db = VecDatabase::new();
    let result = match &cli.command {
        Some(Commands::Add(opts)) => add(&mut db, opts),
        Some(Commands::List(opts)) => list(&db, opts),
        None => Ok(()),
        _ => todo!(),
    };

    match result {
        Ok(_) => {}
        Err(e) => println!("command failed: {}", e),
    }
}

#[derive(Args)]
struct AddOptions {
    #[arg(short, long)]
    name: String,
}

impl Into<Task> for &AddOptions {
    fn into(self) -> Task {
        Task::new(&self.name)
    }
}

#[derive(Args)]
struct ListOptions;

fn add(db: &mut VecDatabase, opts: &AddOptions) -> Result<(), JendaError> {
    db.add_task(opts.into())
}

fn list(db: &VecDatabase, _opts: &ListOptions) -> Result<(), JendaError> {
    let tasks = db.tasks()?;
    let table = Table::new(tasks);
    println!("{}", table);

    Ok(())
}
