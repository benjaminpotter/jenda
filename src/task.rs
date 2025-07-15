use chrono::prelude::*;
use tabled::Tabled;
use uuid::Uuid;

#[derive(Tabled)]
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_completed(&self) -> &bool {
        &self.completed
    }
}
