use crate::{JendaError, Task};

mod sqlite;
mod vec;

pub use sqlite::SqliteDatabase;
pub use vec::VecDatabase;

pub trait Database {
    fn add_task(&mut self, task: Task) -> Result<(), JendaError>;
    fn tasks(&self) -> Result<Tasks, JendaError>;
}

pub struct Tasks<'a> {
    inner: std::slice::Iter<'a, Task>,
}

impl<'a> Iterator for Tasks<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
