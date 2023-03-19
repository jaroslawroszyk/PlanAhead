use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub priority: ESquare,
    pub date: DateTime<Local>,
    pub is_done: bool,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            name: "".to_string(),
            priority: ESquare::Default,
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

    pub fn with_priority(mut self, priority: ESquare) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_date(mut self, date: DateTime<Local>) -> Self {
        self.date = date;
        self
    }

    pub fn toogle_status(&mut self) {
        self.is_done = !self.is_done;
    }
}

impl FromStr for Task {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split(';').map(|arg| arg.trim()).collect();
        let task = match args[..] {
            [name] => Task::new(name),
            [name, priority] => Task::new(name).with_priority(ESquare::from_str(priority)?),
            // TODO: set date
            _ => return Err(()),
        };
        Ok(task)
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        format!("{}; {}", self.name, self.priority.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ESquare {
    ImportantUrgent = 1,
    Important = 2,
    Urgent = 3,
    Default = 4,
}

impl ToString for ESquare {
    fn to_string(&self) -> String {
        match self {
            ESquare::ImportantUrgent => String::from("IU"),
            ESquare::Important => String::from("I"),
            ESquare::Urgent => String::from("U"),
            ESquare::Default => String::from(""),
        }
    }
}

impl FromStr for ESquare {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "UI" | "IU" => Ok(ESquare::ImportantUrgent),
            "I" => Ok(ESquare::Important),
            "U" => Ok(ESquare::Urgent),
            "" => Ok(ESquare::Default),
            _ => Err(()),
        }
    }
}
