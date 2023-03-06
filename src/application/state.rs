use crate::application::Task;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tui::widgets::ListState;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub tasks: StatefulList<Task>,
}

impl State {
    pub fn add_task(&mut self, text: &str) {
        if let Ok(task) = Task::from_str(text) {
            self.tasks.items.push(task);
        }
    }

    pub fn remove_selected_task(&mut self) {
        if let Some(selected) = self.tasks.state.selected() {
            self.tasks.items.remove(selected);
        }
    }

    pub fn clear_all_tasks(&mut self) {
        self.tasks.items.clear();
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct StatefulList<T> {
    #[serde(skip)]
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    pub fn next(&mut self) {
        let current = self.state.selected();
        let next = current.map(|i| (i + 1) % self.items.len());
        self.state.select(Some(next.unwrap_or(0)));
    }

    pub fn previous(&mut self) {
        let current = self.state.selected();
        let previous = current.map(|i| i.checked_sub(1).unwrap_or(self.items.len() - 1));
        self.state.select(Some(previous.unwrap_or(0)));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
