use super::*;

pub struct AddTaskEventHandler;
impl EventHandler for AddTaskEventHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, _: &mut StatefulUi) {
        match key.code {
            KeyCode::Enter => {
                app.add_task();
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
