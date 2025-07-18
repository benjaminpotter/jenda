use crate::{JendaError, Task, TaskGroup};
use chrono::prelude::*;
use rusqlite::Connection;
use std::path::Path;
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, JendaError> {
        let conn = Connection::open(path)
            .map_err(|e| JendaError::Database(format!("failed to open database: {}", e)))?;

        // Create the `task` table if it doesn't exist.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS task (
                id VARCHAR(255) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                complete BOOL NOT NULL,
                timestamp TIMESTAMP NOT NULL
            )",
            (),
        )
        .map_err(|e| JendaError::Database(format!("failed to create table: {}", e)))?;

        Ok(Self { conn })
    }

    pub fn insert(&mut self, task: Task) -> Result<(), JendaError> {
        let timestamp = task.get_timestamp().to_rfc3339();
        match self.conn.execute(
            "INSERT INTO task (id, name, complete, timestamp) VALUES (?1, ?2, ?3, ?4)",
            (
                task.get_id().to_string(),
                task.get_name(),
                task.get_complete(),
                &timestamp,
            ),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(JendaError::Database(format!(
                "failed to insert into table: {}",
                e
            ))),
        }
    }

    pub fn query_id(&self, id: &Uuid) -> Result<Task, JendaError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, complete, timestamp FROM task WHERE id = ?1")
            .map_err(|e| JendaError::Database(format!("failed to prepare statement: {}", e)))?;

        stmt.query_one([id.to_string()], |row| {
            Ok((
                Uuid::parse_str(&row.get::<_, String>(0)?)
                    .expect("to get valid uuid v4 from database"),
                row.get(1)?,
                row.get(2)?,
                row.get::<_, String>(3)?
                    .parse::<DateTime<Utc>>()
                    .expect("to get valid rfc3339 timestamp from database"),
            )
                .into())
        })
        .map_err(|e| JendaError::Database(format!("failed to query table: {}", e)))
    }

    pub fn query(&self, group: &TaskGroup) -> Result<Vec<Task>, JendaError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, complete, timestamp FROM task")
            .map_err(|e| JendaError::Database(format!("failed to prepare statement: {}", e)))?;

        Ok(stmt
            .query([])
            .map_err(|e| {
                JendaError::Database(format!("failed to execute prepared statement: {}", e))
            })?
            .mapped(|row| {
                Ok((
                    Uuid::parse_str(&row.get::<_, String>(0)?)
                        .expect("to get valid uuid v4 from database"),
                    row.get(1)?,
                    row.get(2)?,
                    row.get::<_, String>(3)?
                        .parse::<DateTime<Utc>>()
                        .expect("to get valid rfc3339 timestamp from database"),
                )
                    .into())
            })
            // Warn user that reading a database row failed?
            .filter_map(Result::ok)
            .filter(|task| group.contains(&task))
            .collect())
    }
}
