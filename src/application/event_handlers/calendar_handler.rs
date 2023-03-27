use super::*;

pub struct CalendarEventHandler;
impl EventHandler for CalendarEventHandler {
    fn on_key_press(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.is_running = false,
            KeyCode::Up => ui.previous_week(),
            KeyCode::Down => ui.next_week(),
            KeyCode::Left => ui.previous_day(),
            KeyCode::Right => ui.next_day(),
            KeyCode::Tab => {
                ui.set_date(app.date);
                app.state = State::Default;
            }
            KeyCode::Enter => {
                app.set_date(ui.date);
                app.state = State::Default;
            }
            _ => (),
        };
    }
}
