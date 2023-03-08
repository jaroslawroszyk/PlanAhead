use tui::widgets::ListState;

#[derive(Default)]
pub struct StatefulUi {
    pub tasks: ListState,
}

impl StatefulUi {
    pub fn next_task(&mut self, tasks_len: usize) {
        let current = self.tasks.selected();
        let next = current.map(|i| (i + 1) % tasks_len);
        self.tasks.select(Some(next.unwrap_or(0)));
    }

    pub fn previous_task(&mut self, tasks_len: usize) {
        let current = self.tasks.selected();
        let previous = current.map(|i| i.checked_sub(1).unwrap_or(tasks_len - 1));
        self.tasks.select(Some(previous.unwrap_or(0)));
    }

    pub fn unselect_task(&mut self) {
        self.tasks.select(None);
    }

    pub fn selected_task(&self) -> Option<usize> {
        self.tasks.selected()
    }
}
