use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::application::{App, InputMode};

pub struct EventHandler;
impl EventHandler {
    pub fn on_key(key: KeyEvent, app: &mut App) {
        if key.kind == KeyEventKind::Press {
            Self::on_key_press(key, app)
        }
    }

    fn on_key_press(key: KeyEvent, app: &mut App) {
        match app.input_mode {
            InputMode::Command => Self::handle_command_key_press(key, app),
            InputMode::AddTask => Self::handle_add_task_key_press(key, app),
        };
    }

    fn handle_command_key_press(key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.is_running = false,
            KeyCode::Char('d') => app.state.remove_selected_task(),
            KeyCode::Char('x') => app.state.clear_all_tasks(), //Todo: add prompt for user ("Are you sure?...")
            KeyCode::Up => app.state.tasks.previous(),
            KeyCode::Down => app.state.tasks.next(),
            KeyCode::Left => app.state.tasks.unselect(),
            KeyCode::Char('a') | KeyCode::Char('A') => app.input_mode = InputMode::AddTask,
            _ => (),
        };
    }

    fn handle_add_task_key_press(key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Enter => {
                app.state.add_task(&app.text_input);
                app.text_input = String::new();
                app.input_mode = InputMode::Command;
            }
            KeyCode::Backspace => {
                app.text_input.pop();
            }
            KeyCode::Esc => app.input_mode = InputMode::Command,
            KeyCode::Char(c) => app.text_input.push(c),
            _ => (),
        };
    }
}
