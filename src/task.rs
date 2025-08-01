use chrono::prelude::*;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Task {
    id: Uuid,
    name: String,
    complete: bool,
    timestamp: DateTime<Utc>,
}

impl Task {
    pub fn new(name: &str, complete: bool) -> Self {
        Task {
            id: Uuid::new_v4(),
            name: name.to_string(),
            complete,
            timestamp: Utc::now(),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the first 4 bytes of the task's id.
    ///
    /// The abbreviated task id is easier to work with than its un-abbreviated
    /// counterpart.
    /// It is inspired by abbreviated commit hashes produced by git.
    /// Using the abbreviated task id increases the risk of collision.
    pub fn abrv_id(&self) -> String {
        let mut simple = self.id.simple().to_string();
        simple.truncate(8);
        simple.shrink_to_fit();

        simple
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_complete(&self) -> &bool {
        &self.complete
    }

    /// Returns a short string that represents the complete status of the task.
    ///
    /// | Flag | Description |
    /// +------+-------------+
    /// | COM  | Complete    |
    /// | INC  | Incomplete  |
    pub fn complete_flag(&self) -> &'static str {
        match self.complete {
            true => "COM",
            false => "INC",
        }
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn complete(&mut self) {
        self.complete = true;
    }
}

impl std::cmp::PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<(Uuid, String, bool, DateTime<Utc>)> for Task {
    fn from(tuple: (Uuid, String, bool, DateTime<Utc>)) -> Self {
        Self {
            id: tuple.0,
            name: tuple.1,
            complete: tuple.2,
            timestamp: tuple.3,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}\n",
            self.get_timestamp().with_timezone(&Local).format("%c"),
            self.abrv_id(),
            self.complete_flag(),
            self.get_name()
        )
    }
}

/// TaskGroup { name: None, complete: None } matches all tasks.
pub struct TaskGroup {
    name: Option<String>,
    complete: Option<bool>,
}

impl TaskGroup {
    pub fn new(name: Option<String>, complete: Option<bool>) -> Self {
        Self { name, complete }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_complete(mut self, complete: bool) -> Self {
        self.complete = Some(complete);
        self
    }

    pub fn contains(&self, task: &Task) -> bool {
        let matches_name = match self.name {
            Some(ref name) => task.name.contains(name),
            None => true,
        };

        let matches_complete = match self.complete {
            Some(complete) => complete == task.complete,
            None => true,
        };

        matches_name && matches_complete
    }
}
