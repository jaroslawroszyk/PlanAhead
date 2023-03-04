use chrono::{DateTime, Local};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub name: String,
    pub priority: u8,
    pub date: DateTime<Local>,
}

impl Task {
    pub fn new(name: &str, priority: Option<u8>, date: Option<DateTime<Local>>) -> Task {
        Task {
            name: name.to_string(),
            priority: priority.unwrap_or(1),
            date: date.unwrap_or(Local::now()),
        }
    }

    pub fn from_string(str: &str) -> Option<Task> {
        let args: Vec<&str> = str.split(';').map(|arg| arg.trim()).collect();
        match args.len() {
            1 => Some(Task::new(args[0], None, None)),
            2 => {
                let priority: u8 = args[1].parse().unwrap(); // TODO: handle failure
                Some(Task::new(args[0], Some(priority), None))
            }
            // TODO: set date
            _ => None,
        }
    }
}
