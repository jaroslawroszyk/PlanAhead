use super::*;

pub struct PromptEventHandler;
impl EventHandler for PromptEventHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, _: &mut StatefulUi) {
        match key.code {
            KeyCode::Enter => {
                app.confirm_previous_action();
                app.previous_action = None;
                app.state = State::Default;
            }
            KeyCode::Esc => {
                app.state = State::Default;
                app.previous_action = None;
            }
            _ => (),
        };
    }
}
