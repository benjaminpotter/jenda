use crate::{
    db::{Database, Tasks},
    JendaError, Task,
};
use rusqlite::Connection;

pub struct SqliteDatabase {
    conn: Connection,
}

impl Database for SqliteDatabase {
    fn add_task(&mut self, task: Task) -> Result<(), JendaError> {
        Ok(())
    }

    fn tasks(&self) -> Result<Tasks, JendaError> {
        Ok(Tasks { inner: [].iter() })
    }
}
