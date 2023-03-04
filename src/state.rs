use anyhow::Result;
use tui::widgets::ListState;

use crate::tasks::Task;

use std::fs::{self, File, OpenOptions};

pub struct State {
    pub text_input: String,
    pub tasks: StatefulList<Task>,
}

impl Default for State {
    fn default() -> Self {
        State {
            text_input: String::new(),
            tasks: StatefulList::with_items(Vec::<Task>::new()),
        }
    }
}

impl State {
    pub fn load_tasks(&mut self) -> Result<()> {
        let file = open_file(FileMode::Load)?;
        let tasks: Vec<Task> = serde_json::from_reader(file)?;
        self.tasks = StatefulList::with_items(tasks);
        Ok(())
    }

    pub fn save_tasks(&self) -> Result<()> {
        let file = open_file(FileMode::Save)?;
        serde_json::to_writer_pretty(file, &self.tasks.items)?;
        Ok(())
    }

    pub fn add_task(&mut self) {
        let task = Task::from_string(&self.text_input).unwrap(); // TODO: handle failure
        self.tasks.items.push(task);
        self.save_tasks()
            .expect("error: could not save tasks to a file"); // TODO: handle failure
        self.text_input = String::new();
    }
}

enum FileMode {
    Load,
    Save,
}

fn open_file(mode: FileMode) -> Result<File> {
    let file = match mode {
        FileMode::Load => {
            let f = OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open("tasks.json")?;
            if fs::metadata("tasks.json")?.len() == 0 {
                let default_content: Vec<Task> = Vec::new();
                fs::write(
                    "tasks.json",
                    serde_json::to_string_pretty(&default_content)?,
                )?
            };
            f
        }
        FileMode::Save => OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("tasks.json")?,
    };
    Ok(file)
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
