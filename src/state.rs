use crate::tasks::Task;
use anyhow::Result;
use chrono::{DateTime, Local};
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

    pub fn save_tasks(&self) -> Result<()> {
        let file = open_file()?;
        serde_json::to_writer_pretty(file, &self.tasks)?;
        Ok(())
    }

    pub fn add_task(
        &mut self,
        name: &str,
        priority: Option<u8>,
        date: Option<DateTime<Local>>,
    ) -> Result<()> {
        let task = Task::new(name, priority, date);
        self.tasks.push(task);
        self.save_tasks().ok();
        Ok(())
    }

    //Todo: Delete this function when all implementation will be ready.
    fn placeholder_save_to_json(&mut self, msg: &str) -> Result<()> {
        let task = Task::new(msg, Some(1), None);
        self.tasks.push(task);
        self.save_tasks().ok();
        Ok(())
    }

    pub fn edit_task(&mut self) -> Result<()> {
        self.placeholder_save_to_json("edit_task").ok();
        Ok(())
    }
    pub fn modify_task(&mut self) -> Result<()> {
        self.placeholder_save_to_json("modify_task").ok();
        Ok(())
    }
    pub fn complete_task(&mut self) -> Result<()> {
        self.placeholder_save_to_json("complete_task").ok();
        Ok(())
    }
    pub fn delete_task(&mut self) -> Result<()> {
        self.placeholder_save_to_json("delete_task").ok();
        Ok(())
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