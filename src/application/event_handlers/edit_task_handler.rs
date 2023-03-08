use super::*;

pub struct EditTaskEventHandler;
impl EventHandler for EditTaskEventHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match key.code {
            KeyCode::Enter => {
                app.edit_task(ui.selected_task());
                app.input = String::new();
                app.state = State::Default;
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => app.state = State::Default,
            KeyCode::Char(c) => app.input.push(c),
            _ => (),
        };
    }
}
