use super::Action;
use crate::{
    application::{App, InputMode},
    ui::StatefulUi,
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub struct EventHandler;
impl EventHandler {
    pub fn on_key(key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        if key.kind == KeyEventKind::Press {
            Self::on_key_press(key, app, ui)
        }
    }

    fn on_key_press(key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match app.input_mode {
            InputMode::Command => Self::handle_command_key_press(key, app, ui),
            InputMode::AddTask => Self::handle_add_task_key_press(key, app, ui),
            InputMode::Prompt => Self::handle_prompt_key_press(key, app, ui),
        };
    }

    fn handle_command_key_press(key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.is_running = false,
            KeyCode::Char('d') => app.remove_selected_task(ui.selected_task()),
            KeyCode::Char('x') => {
                app.input_mode = InputMode::Prompt;
                app.previous_action = Some(Action::ClearAllTasks);
            }
            KeyCode::Up => ui.previous_task(app.tasks.len()),
            KeyCode::Down => ui.next_task(app.tasks.len()),
            KeyCode::Left => ui.unselect_task(),
            KeyCode::Char('a') | KeyCode::Char('A') => app.input_mode = InputMode::AddTask,
            _ => (),
        };
    }

    fn handle_add_task_key_press(key: KeyEvent, app: &mut App, _: &mut StatefulUi) {
        match key.code {
            KeyCode::Enter => {
                app.add_task();
                app.input = String::new();
                app.input_mode = InputMode::Command;
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => app.input_mode = InputMode::Command,
            KeyCode::Char(c) => app.input.push(c),
            _ => (),
        };
    }

    fn handle_prompt_key_press(key: KeyEvent, app: &mut App, _: &mut StatefulUi) {
        match key.code {
            KeyCode::Enter => {
                app.confirm_previous_action();
                app.previous_action = None;
                app.input_mode = InputMode::Command;
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Command;
                app.previous_action = None;
            }
            _ => (),
        };
    }
}
