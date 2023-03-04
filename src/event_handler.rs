use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub struct EventHandler;

impl EventHandler {
    pub fn on_key(key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Char('a') => app.state.add_task("test", Some(1), None),
            KeyCode::Char('e') => app.state.edit_task(),
            KeyCode::Char('m') => app.state.modify_task(),
            KeyCode::Enter => app.state.complete_task(),
            KeyCode::Char('d') => app.state.delete_task(),
            KeyCode::Esc | KeyCode::Char('q') => {
                app.is_running = false;
                Ok(())
            }
            _ => Ok(()),
        }
        .ok();
    }
}
