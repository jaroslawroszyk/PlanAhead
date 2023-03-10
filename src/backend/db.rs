use crate::application::tasks::Task;
use anyhow::Result;
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

const DB_PATH: &str = "tasks.json";

pub fn load() -> Result<Vec<Task>> {
    let file = OpenOptions::new().read(true).open(DB_PATH)?;
    let reader = BufReader::new(file);
    let state = serde_json::from_reader(reader)?;
    Ok(state)
}

pub fn save(state: &[Task]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DB_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, state)?;
    Ok(())
}
