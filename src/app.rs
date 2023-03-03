pub struct App {
    pub is_running: bool,
}

impl Default for App {
    fn default() -> Self {
        App { is_running: true }
    }
}
