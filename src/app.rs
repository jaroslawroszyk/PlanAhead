use crate::state::State;

pub struct App {
    pub is_running: bool,
    pub state: State,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            state: State::default(),
        }
    }
}
