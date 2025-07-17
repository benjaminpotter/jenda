use crate::JendaError;
use chrono::prelude::*;
use tabled::Tabled;
use uuid::Uuid;

#[derive(Clone, Debug, Tabled)]
pub struct Task {
    id: Uuid,
    name: String,
    completed: bool,
    timestamp: DateTime<Utc>,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Task {
            id: Uuid::new_v4(),
            name: name.to_string(),
            completed: false,
            timestamp: Utc::now(),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_completed(&self) -> &bool {
        &self.completed
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
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
            completed: tuple.2,
            timestamp: tuple.3,
        }
    }
}

pub struct Tasks {
    inner: Vec<Result<Task, JendaError>>,
}
