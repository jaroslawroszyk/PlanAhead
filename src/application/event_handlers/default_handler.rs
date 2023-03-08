use super::*;

pub struct DefaultEventHandler;
impl EventHandler for DefaultEventHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.is_running = false,
            KeyCode::Char('d') => app.remove_selected_task(ui.selected_task()),
            KeyCode::Char('x') => {
                app.state = State::Prompt;
                app.previous_action = Some(Action::ClearAllTasks);
            }
            KeyCode::Up => ui.previous_task(app.tasks.len()),
            KeyCode::Down => ui.next_task(app.tasks.len()),
            KeyCode::Left => ui.unselect_task(),
            KeyCode::Char('a') | KeyCode::Char('A') => app.state = State::AddTask,
            KeyCode::Char('e') | KeyCode::Char('E') => {
                app.state = State::EditTask;
                if let Some(idx) = ui.selected_task() {
                    app.input = app.tasks[idx].to_string()
                }
            }
            _ => (),
        };
    }
}
