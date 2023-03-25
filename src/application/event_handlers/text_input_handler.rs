use super::*;

pub struct TextInputHandler;
impl EventHandler for TextInputHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match key.code {
            KeyCode::Enter => {
                self.finish_action(app, ui);
                app.input.clear();
                app.state = State::Default;
                ui.cursor_offset = 0;
            }
            KeyCode::Backspace => {
                let idx = app.input.len().saturating_sub(ui.cursor_offset);
                if idx > 0 && idx <= app.input.len() {
                    app.input.remove(idx - 1);
                }
            }
            KeyCode::Esc => {
                app.input.clear();
                app.state = State::Default;
                ui.cursor_offset = 0;
            }
            KeyCode::Left => ui.cursor_offset = (ui.cursor_offset + 1).min(app.input.len()),
            KeyCode::Right => ui.cursor_offset = ui.cursor_offset.saturating_sub(1),
            KeyCode::Char(c) => {
                let idx = app.input.len() - ui.cursor_offset;
                app.input.insert(idx, c);
            }
            _ => (),
        };
    }
}

impl TextInputHandler {
    fn finish_action(&self, app: &mut App, ui: &mut StatefulUi) {
        match app.state {
            State::AddTask => app.add_task(),
            State::EditTask => app.edit_task(ui.selected_task()),
            _ => unreachable!(),
        }
    }
}
