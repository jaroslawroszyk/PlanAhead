use crate::{application::State, backend::db};

#[derive(Clone, Copy)]
pub enum InputMode {
    Command,
    AddTask,
}

pub struct App {
    pub is_running: bool,
    pub state: State,
    pub text_input: String,
    pub input_mode: InputMode,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            state: db::load().unwrap_or_default(),
            text_input: String::new(),
            input_mode: InputMode::Command,
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
