use crate::state::State;

pub enum InputMode {
    Command,
    AddTask,
}

pub struct App {
    pub is_running: bool,
    pub state: State,
    pub input_mode: InputMode,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            state: State::default(),
            input_mode: InputMode::Command,
        }
    }
}
