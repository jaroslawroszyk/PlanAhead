use chrono::{DateTime, Local};

use crate::tasks::Task;

use std::fs::{File, OpenOptions};
pub struct State {
    tasks: Vec<Task>,
}

impl Default for State {
    fn default() -> Self {
        State { tasks: Vec::new() }
    }
}

impl State {
    pub fn load_tasks(&mut self) -> std::io::Result<()> {
        let file = open_file()?;
        let tasks: Vec<Task> = serde_json::from_reader(file)?;
        self.tasks = tasks;
        Ok(())
    }

    pub fn save_tasks(&self) -> std::io::Result<()> {
        let file = open_file()?;
        serde_json::to_writer_pretty(file, &self.tasks)?;
        Ok(())
    }

    pub fn add_task(&mut self, name: &str, priority: Option<u8>, date: Option<DateTime<Local>>) {
        let task = Task::new(name, priority, date);
        self.tasks.push(task);
    }
}

fn open_file() -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.json")
}
