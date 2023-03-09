use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub priority: u8,
    pub date: DateTime<Local>,
    pub is_done: bool,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            name: "".to_string(),
            priority: 1,
            date: Local::now(),
            is_done: false,
        }
    }
}

impl Task {
    pub fn new(name: &str) -> Self {
        Task {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_date(mut self, date: DateTime<Local>) -> Self {
        self.date = date;
        self
    }

    pub fn change_status(&mut self, is_done: bool) {
        self.is_done = is_done;
    }
}

impl FromStr for Task {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split(';').map(|arg| arg.trim()).collect();
        let task = match args[..] {
            [name] => Task::new(name),
            [name, priority] => {
                let priority: u8 = priority.parse().map_err(|_| ())?;
                Task::new(name).with_priority(priority)
            }
            // TODO: set date
            _ => return Err(()),
        };
        Ok(task)
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        format!("{}; {}", self.name, self.priority)
    }
}
