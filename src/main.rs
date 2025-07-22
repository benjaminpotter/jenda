use clap::{Args, Parser, Subcommand};
use jenda::{Database, JendaError, Task, TaskGroup};
use std::{fs, path::PathBuf};
use uuid::Uuid;

// jenda add -n "cargo update"
// jenda complete -n "cargo update"
// jenda list --incomplete
// jenda list
// jenda list --filter "complete=false"

#[derive(Parser)]
struct JendaCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task.
    Add(AddOptions),

    /// Mark all tasks in group as complete.
    Ct(CtOptions),

    /// List all tasks in a task group.
    List(ListOptions),

    /// Display info for a single task.
    Info(InfoOptions),
}

fn main() {
    let cli = JendaCli::parse();
    match run(cli) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("command failed: {}", e),
    }
}

fn run(cli: JendaCli) -> Result<String, JendaError> {
    let mut path = match std::env::var("XDG_DATA_HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => [
            &std::env::var("HOME").expect("set HOME env var"),
            ".local",
            "share",
        ]
        .iter()
        .collect(),
    };

    path.push("jenda");
    fs::create_dir_all(&path).expect("failed to create app dir under XDG_DATA_HOME");

    path.push("task.db");
    let mut db = Database::open(path)?;

    match &cli.command {
        Some(Commands::Add(opts)) => add(&mut db, &opts),
        Some(Commands::Ct(opts)) => ct(&mut db, &opts),
        Some(Commands::List(opts)) => list(&db, &opts),
        Some(Commands::Info(opts)) => info(&db, &opts),
        None => Ok(String::new()),
    }
}

#[derive(Args)]
struct AddOptions {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    complete: bool,
}

impl Into<Task> for &AddOptions {
    fn into(self) -> Task {
        Task::new(&self.name, self.complete)
    }
}

fn add(db: &mut Database, opts: &AddOptions) -> Result<String, JendaError> {
    db.insert(opts.into())?;
    Ok(String::new())
}

#[derive(Args)]
struct CtOptions {
    /// Retains tasks that contain `name` as a substring of `task.name`.
    #[arg(short, long)]
    name: Option<String>,
}

fn ct(db: &Database, opts: &CtOptions) -> Result<String, JendaError> {
    let group = TaskGroup::new(opts.name.clone(), Some(false));

    // Query db for matching tasks.
    let tasks = db.query(&group)?;
    let cnt = tasks.len();

    // Complete task and update db.
    for mut task in tasks {
        task.complete();
        db.update(task)?;
    }

    Ok(format!("marked {} tasks as complete", cnt))
}

#[derive(Args)]
struct ListOptions {
    /// Retains tasks that contain `name` as a substring of `task.name`.
    #[arg(short, long)]
    name: Option<String>,

    /// Retains tasks that have `task.complete` set to `false`.
    #[arg(short, long)]
    incomplete: bool,
}

fn list(db: &Database, opts: &ListOptions) -> Result<String, JendaError> {
    let mut group = TaskGroup::new(opts.name.clone(), None);

    if opts.incomplete {
        // If the user includes the incomplete flag (its true) then we are
        // looking for tasks without the complete flag (its false).
        group = group.with_complete(false);
    }

    Ok(db
        .query(&group)?
        .into_iter()
        .map(|task| format!("{}\n", task))
        .collect())
}

#[derive(Args)]
struct InfoOptions {
    id: Uuid,
}

fn info(db: &Database, opts: &InfoOptions) -> Result<String, JendaError> {
    let task = db.query_id(&opts.id)?;
    Ok(format!("{}", task))
}
