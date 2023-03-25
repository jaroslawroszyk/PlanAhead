use chrono::{Local, NaiveDate, Days};
use tui::widgets::ListState;

pub struct StatefulUi {
    pub tasks: ListState,
    pub cursor_offset: usize,
    pub date: NaiveDate,
}

impl Default for StatefulUi {
    fn default() -> Self {
        StatefulUi {
            tasks: ListState::default(),
            cursor_offset: 0,
            date: Local::now().date_naive(),
        }
    }
}

impl StatefulUi {
    pub fn next_task(&mut self, tasks_len: usize) {
        if tasks_len == 0 {
            return;
        }

        let current = self.tasks.selected();
        let next = current.map(|i| (i + 1) % tasks_len);
        self.tasks.select(Some(next.unwrap_or(0)));
    }

    pub fn previous_task(&mut self, tasks_len: usize) {
        if tasks_len == 0 {
            return;
        }

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

    pub fn next_day(&mut self) {
        self.date = self.date.succ_opt().unwrap()
    }

    pub fn previous_day(&mut self) {
        self.date = self.date.pred_opt().unwrap();
    }

    pub fn next_week(&mut self) {
        self.date = self.date.checked_add_days(Days::new(7)).unwrap();
    }

    pub fn previous_week(&mut self) {
        self.date = self.date.checked_sub_days(Days::new(7)).unwrap();
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }
}
