use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub struct EventHandler;

impl EventHandler {
    pub fn on_key(key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => app.is_running = false,
            _ => (),
        }
    }
}
