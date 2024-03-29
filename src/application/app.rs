use crate::{
    application::{State, Task},
    backend::db,
};
use chrono::{Local, NaiveDate};
use std::str::FromStr;

#[derive(PartialEq)]
pub enum Action {
    ClearAllTasks,
}

pub struct App {
    pub is_running: bool,
    pub tasks: Vec<Task>,
    pub state: State,
    pub input: String,
    pub previous_action: Option<Action>,
    pub date: NaiveDate,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            tasks: db::load().unwrap_or_default(),
            state: State::Default,
            input: String::default(),
            previous_action: None,
            date: Local::now().date_naive(),
        }
    }
}

impl App {
    pub fn confirm_previous_action(&mut self) {
        if self.previous_action == Some(Action::ClearAllTasks) {
            self.clear_all_tasks();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        db::save(&self.tasks).unwrap_or_else(|err| {
            eprintln!("Faild to save app state: {err}");
        });
    }
}

impl App {
    pub fn add_task(&mut self) {
        if let Ok(task) = Task::from_str(&self.input) {
            self.tasks.push(task);
        }
    }

    pub fn edit_task(&mut self, idx: Option<usize>) {
        if let Some(i) = idx {
            if let Ok(task) = Task::from_str(&self.input) {
                self.tasks[i] = task;
            }
        }
    }

    pub fn remove_selected_task(&mut self, selected: Option<usize>) {
        if let Some(selected) = selected {
            self.tasks.remove(selected);
        }
    }

    pub fn clear_all_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn change_task_status(&mut self, selected: Option<usize>) {
        if let Some(idx) = selected {
            let current_task = &mut self.tasks[idx];
            current_task.toogle_status();
        }
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }
}
