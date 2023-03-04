use crate::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub struct EventHandler;

impl EventHandler {
    pub fn on_key(key: KeyEvent, app: &mut App) {
        if key.kind == KeyEventKind::Press {
            match app.input_mode {
                InputMode::Command => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        app.is_running = false
                    }
                    KeyCode::Up => app.state.tasks.previous(),
                    KeyCode::Down => app.state.tasks.next(),
                    KeyCode::Left => app.state.tasks.unselect(),
                    KeyCode::Char('a') | KeyCode::Char('A') => app.input_mode = InputMode::AddTask,
                    _ => (),
                },
                InputMode::AddTask => match key.code {
                    KeyCode::Enter => {
                        app.state.add_task();
                        app.input_mode = InputMode::Command;
                    }
                    KeyCode::Char(c) => {
                        app.state.text_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.state.text_input.pop();
                    }
                    KeyCode::Esc => app.input_mode = InputMode::Command,
                    _ => (),
                },
            }
        }
    }
}
