use chrono::{DateTime, Local};

use crate::tasks::Task;

use std::fs::{self, File, OpenOptions};

#[derive(Default)]
pub struct State {
    tasks: Vec<Task>,
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
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json")?;
    if fs::metadata("tasks.json")?.len() == 0 {
        let default_content: Vec<Task> = Vec::new();
        fs::write(
            "tasks.json",
            serde_json::to_string_pretty(&default_content)?,
        )?
    };
    Ok(file)
}
