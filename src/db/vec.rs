use crate::{
    db::{Database, Tasks},
    JendaError, Task,
};

pub struct VecDatabase {
    inner: Vec<Task>,
}

impl VecDatabase {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl Database for VecDatabase {
    fn add_task(&mut self, task: Task) -> Result<(), JendaError> {
        self.inner.push(task);
        Ok(())
    }

    fn tasks(&self) -> Result<Tasks, JendaError> {
        Ok(Tasks {
            inner: self.inner.iter(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut vdb = VecDatabase::new();
        let task = Task::new("sudo dnf update");
        vdb.add_task(task.clone()).unwrap();
        assert_eq!(vdb.inner, vec![task]);
    }

    #[test]
    fn test_tasks_iter() {
        let task = Task::new("sudo dnf update");
        let mut vdb = VecDatabase {
            inner: vec![task.clone()],
        };

        let other = vdb.tasks().unwrap().next().unwrap();
        assert_eq!(other, &task);
    }
}
