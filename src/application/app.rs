use crate::{application::State, backend::db};

#[derive(Clone, Copy)]
pub enum InputMode {
    Command,
    AddTask,
    Prompt,
}

#[derive(PartialEq)]
pub enum Action {
    ClearAllTasks,
}

pub struct App {
    pub is_running: bool,
    pub state: State,
    pub text_input: String,
    pub input_mode: InputMode,
    pub previous_action: Option<Action>,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            state: db::load().unwrap_or_default(),
            text_input: String::new(),
            input_mode: InputMode::Command,
            previous_action: None,
        }
    }
}

impl App {
    pub fn confirm_previous_action(&mut self) {
        if self.previous_action == Some(Action::ClearAllTasks) {
            self.state.clear_all_tasks();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        db::save(&self.state).unwrap_or_else(|err| {
            eprintln!("Faild to save app state: {err}");
        });
    }
}
