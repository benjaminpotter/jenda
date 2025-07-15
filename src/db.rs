use crate::{JendaError, Task};

pub struct Database {
    inner: Vec<Task>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            inner: vec![Task::new("sudo dnf update")],
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), JendaError> {
        self.inner.push(task);
        Ok(())
    }

    pub fn tasks(&self) -> Result<Tasks, JendaError> {
        Ok(Tasks {
            inner: self.inner.iter(),
        })
    }
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
