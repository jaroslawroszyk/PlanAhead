use num_traits::clamp;

use super::*;

pub struct DefaultEventHandler;
impl EventHandler for DefaultEventHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.is_running = false,
            KeyCode::Char('d') | KeyCode::Char('D') => app.remove_selected_task(ui.selected_task()),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                app.state = State::Prompt;
                app.previous_action = Some(Action::ClearAllTasks);
                ui.unselect_task();
            }
            KeyCode::Up => ui.previous_task(app.tasks.len()),
            KeyCode::Down => ui.next_task(app.tasks.len()),
            KeyCode::Left => ui.unselect_task(),
            KeyCode::Char('a') | KeyCode::Char('A') => app.state = State::AddTask,
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if let Some(mut idx) = ui.selected_task() {
                    idx = clamp(idx, 0, app.tasks.len());
                    app.input = app.tasks[idx].to_string();
                    app.state = State::EditTask;
                }
            }
            KeyCode::Tab => app.state = State::Calendar,
            KeyCode::Enter => app.change_task_status(ui.selected_task()),
            _ => (),
        };
    }
}
