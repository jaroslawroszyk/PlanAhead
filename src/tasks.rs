use chrono::{DateTime, Local};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Task {
    name: String,
    priority: u8,
    date: DateTime<Local>,
}

impl Task {
    pub fn new(name: &str, priority: Option<u8>, date: Option<DateTime<Local>>) -> Task {
        Task {
            name: name.to_string(),
            priority: priority.unwrap_or(1),
            date: date.unwrap_or(Local::now()),
        }
    }
}
