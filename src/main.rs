use clap::{Args, Parser, Subcommand};
use jenda::{Database, JendaError, Task};
use tabled::Table;
use uuid::Uuid;

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
    Info(InfoOptions),

    /// Manage configuration options.
    Config,
}

fn main() {
    let cli = JendaCli::parse();
    match run(cli) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("command failed: {}", e),
    }
}

fn run(cli: JendaCli) -> Result<String, JendaError> {
    let mut db = Database::open("testing.db")?;
    match &cli.command {
        Some(Commands::Add(opts)) => add(&mut db, &opts),
        Some(Commands::List(opts)) => list(&db, &opts),
        Some(Commands::Info(opts)) => info(&db, &opts),
        None => Ok(String::new()),
        _ => todo!(),
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

fn add(db: &mut Database, opts: &AddOptions) -> Result<String, JendaError> {
    db.insert(opts.into())?;
    Ok(String::new())
}

#[derive(Args)]
struct ListOptions;

fn list(db: &Database, _opts: &ListOptions) -> Result<String, JendaError> {
    Ok(String::new())
}

#[derive(Args)]
struct InfoOptions {
    id: Uuid,
}

fn info(db: &Database, opts: &InfoOptions) -> Result<String, JendaError> {
    let task = db.query_id(&opts.id)?;
    let table = Table::new(vec![task]);
    Ok(table.to_string())
}
